use crate::please::snippet_and_file;
use crate::utils::{env_term_program, env_term_version};

pub fn print_guide(shell: &str) {
    let pkg = env!("CARGO_PKG_NAME");

    print_title("Installation");
    let instructions = installation_instructions(shell, pkg);
    println!("{instructions}\n");

    if let Some(terminal_tips) = terminal_emulator_tips() {
        print_title("Notes about your terminal emulator");
        println!("{terminal_tips}\n");
    }
}

fn print_title(title: &str) {
    let capitalized = title.to_uppercase();
    println!("\x1b[32;4m{capitalized}\x1b[0m\n");
}

fn installation_instructions(shell: &str, cmd: &str) -> String {
    if let Some((snippet, file)) = snippet_and_file(shell, cmd) {
        format!(
            "To use {cmd} as your {shell} prompt, put this line at the end of ~/{file}:\n\n\x1b[1m{snippet}\x1b[0m"
        )
    } else {
        format!(
            "No help available for {shell}. Check {shell}'s docs to see how to set your prompt. \
            Tip: to make {cmd} render a prompt, just call it with no arguments."
        )
    }
}

pub fn terminal_emulator_tips() -> Option<&'static str> {
    let terminal = env_term_program();
    let terminal_version = env_term_version();

    match (terminal.as_deref(), terminal_version) {
        (Some("Apple_Terminal"), Some(version)) if version.major < 460 => Some(
            "Before macOS Tahoe, Apple's Terminal.app has poor support for fancy prompts. Its color support \
            is limited, and it does not render special powerline characters properly.",
        ),
        (Some("WarpTerminal"), _) => Some(
            "By default, Warp uses its own prompt. See the Warp documentation on how to enable a custom prompt.",
        ),
        (Some("iTerm.app"), _) => Some(
            "To make sure that iTerm draws the prompt correctly, enable: Settings → Profiles → Text → Use built-in Powerline glyphs",
        ),
        _ => None,
    }
}
