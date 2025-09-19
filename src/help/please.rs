use crate::gfx::{bold, green};
use crate::help::terminal_emulator_tips;
use crate::utils::{env_home, yolo_mode};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use std::{env, process};

pub fn please(shell: &str) {
    match update_shell_rc(shell) {
        Ok(msg) => println!("✅ {}", msg),
        Err(e) => {
            eprintln!("🆘 Error: {}", e);
            process::exit(1);
        }
    }

    if let Some(terminal_tips) = terminal_emulator_tips() {
        println!("🔔 {terminal_tips}\n");
    }
}

pub fn update_shell_rc(shell: &str) -> Result<String, String> {
    let pkg = env!("CARGO_PKG_NAME");
    let home = env_home()
        .map(PathBuf::from)
        .ok_or("Could not determine your home directory")?;

    println!("📦 Setting up {pkg} as your shell prompt");
    println!("📦 Your shell seems to be {}", bold(shell));

    let (snippet, filename) = snippet_and_file(shell, pkg)
        .ok_or_else(|| format!("The {} shell is not supported", bold(shell)))?;

    let path = home.join(filename);

    if contains_snippet(&path, &snippet).map_err(|e| e.to_string())? {
        return Ok(format!(
            "Your ~/{filename} already has the required line. The prompt should appear in your next terminal session."
        ));
    }

    if !confirm(&format!(
        "📦 Adding the {} prompt line to your ~/{}…",
        bold(shell),
        bold(filename)
    ))
    .unwrap_or(false)
    {
        return Ok("Nothing changed.".to_string());
    }
    append_snippet(&path, &snippet).map_err(|e| e.to_string())?;

    Ok("Done!".to_string())
}

fn contains_snippet(path: &Path, snippet: &str) -> io::Result<bool> {
    match File::open(path) {
        Ok(f) => Ok(file_contains_snippet(&f, snippet)),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e),
    }
}

fn file_contains_snippet(f: &File, snippet: &str) -> bool {
    io::BufReader::new(f)
        .lines()
        .map_while(Result::ok)
        .any(|line| line.trim().starts_with(snippet))
}

fn append_snippet(path: &Path, snippet: &str) -> io::Result<()> {
    let pkg = env!("CARGO_PKG_NAME");
    let mut f = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(f, "\n{} # Added by {pkg}", snippet)
}

fn confirm(prompt: &str) -> io::Result<bool> {
    println!("{prompt}");

    if yolo_mode() {
        return Ok(true);
    }

    let prelude = green("Press Enter to continue or type \"no\" to cancel");

    loop {
        print!("{prelude}: ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        match input.trim().to_ascii_lowercase().as_str() {
            "no" | "n" => return Ok(false),
            "" => return Ok(true),
            _ => {}
        }
    }
}

pub fn snippet_and_file(shell: &str, cmd: &str) -> Option<(String, &'static str)> {
    match shell {
        "bash" => Some((format!("eval \"$({cmd} init bash)\""), ".bashrc")),
        "zsh" => Some((format!("eval \"$({cmd} init zsh)\""), ".zshrc")),
        "fish" => Some((
            format!("{cmd} init fish | source"),
            ".config/fish/config.fish",
        )),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{NamedTempFile, TempDir};

    #[test]
    fn test_append() -> io::Result<()> {
        let file = NamedTempFile::new()?;
        let path = file.path();
        let (snippet, _) = snippet_and_file("zsh", "gigawatt").unwrap();

        assert!(!contains_snippet(path, &snippet)?);
        append_snippet(path, &snippet)?;
        assert!(contains_snippet(path, &snippet)?);

        Ok(())
    }

    #[test]
    fn test_append_to_nonexistent_file() -> io::Result<()> {
        let dir = TempDir::new()?;
        let path = dir.path().join("new");
        let (snippet, _) = snippet_and_file("zsh", "gigawatt").unwrap();

        assert!(!contains_snippet(&path, &snippet)?);
        append_snippet(&path, &snippet)?;
        assert!(contains_snippet(&path, &snippet)?);

        Ok(())
    }

    #[test]
    fn test_append_to_config_file_without_last_newline() -> io::Result<()> {
        let mut file = NamedTempFile::new()?;
        write!(file, "something")?;

        let path = file.path();
        let (snippet, _) = snippet_and_file("zsh", "gigawatt").unwrap();

        assert!(!contains_snippet(path, &snippet)?);
        append_snippet(path, &snippet)?;
        assert!(contains_snippet(path, &snippet)?);

        Ok(())
    }

    #[test]
    fn test_already_contains() -> io::Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "\t eval \"$(gigawatt init zsh)\" # Foo")?;

        let (snippet, _) = snippet_and_file("zsh", "gigawatt").unwrap();
        assert!(contains_snippet(file.path(), &snippet)?);

        Ok(())
    }

    #[test]
    fn test_commented_snippet_does_not_count() -> io::Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "# eval \"$(gigawatt init zsh)\"")?;

        let (snippet, _) = snippet_and_file("zsh", "gigawatt").unwrap();
        assert!(!contains_snippet(file.path(), &snippet)?);

        Ok(())
    }
}
