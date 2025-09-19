use nix::NixPath;
use std::env;
use std::path::{Component, Path, PathBuf};

pub fn displayed_path(repo_workdir: Option<&Path>) -> Option<String> {
    let path = current_path()?;
    let home = home_path()?;
    short_path(&path, &home, repo_workdir)
}

fn short_path(path: &Path, home: &Path, repo_workdir: Option<&Path>) -> Option<String> {
    if let Some(workdir) = repo_workdir
        && let Some(workdir_parent) = workdir.parent()
        && path.starts_with(workdir)
        && let Ok(rel) = path.strip_prefix(workdir_parent)
    {
        return Some(shorten_path(rel, Some("…")));
    }

    if let Ok(rel) = path.strip_prefix(home) {
        return Some(shorten_path(rel, Some("~")));
    }

    Some(shorten_path(path, None))
}

fn shorten_path(path: &Path, prefix: Option<&str>) -> String {
    if path.is_empty() {
        return prefix.unwrap_or("?").into();
    }

    if directory_depth(path) > 3 {
        let last_three = path
            .components()
            .skip(path.components().count().saturating_sub(3))
            .collect::<PathBuf>();
        return PathBuf::from("…").join(last_three).to_string_lossy().into();
    }

    match prefix {
        Some(p) => PathBuf::from(p).join(path).to_string_lossy().into(),
        None => path.to_string_lossy().into(),
    }
}

// A path like "/tmp" contains 2 components: "/" and "tmp". When truncating long paths, we only
// want to count the normal directories, treating both "/a/b/c" and "a/b/c" as 3 levels.
fn directory_depth(path: &Path) -> usize {
    path.components()
        .filter(|c| matches!(c, Component::Normal(_)))
        .count()
}

pub fn file_name(path: &str) -> Option<String> {
    Path::new(path)
        .file_name()
        .map(|f| f.to_string_lossy().into_owned())
}

pub fn home_path() -> Option<PathBuf> {
    env::var_os("HOME").map(PathBuf::from)
}

pub fn current_path() -> Option<PathBuf> {
    // env::current_dir follows symlinks, so try to use PWD if it seems valid
    let cwd = env::current_dir().ok()?;
    if let Some(env_os) = env::var_os("PWD") {
        let pwd = PathBuf::from(env_os);
        if pwd.exists()
            && let Ok(pwd_canonical) = pwd.canonicalize()
            && let Ok(cwd_canonical) = cwd.canonicalize()
            && pwd_canonical == cwd_canonical
        {
            return Some(pwd);
        }
    }

    Some(cwd)
}

#[test]
fn test_short_path_inner() {
    assert_eq!(
        short_path(&PathBuf::from("/"), &PathBuf::from("/home"), None),
        Some("/".into())
    );
    assert_eq!(
        short_path(&PathBuf::from("/a/b/c/d"), &PathBuf::from("/home"), None),
        Some("…/b/c/d".into())
    );
    assert_eq!(
        short_path(&PathBuf::from("/home/a"), &PathBuf::from("/home"), None),
        Some("~/a".into())
    );
    assert_eq!(
        short_path(
            &PathBuf::from("/home/repo"),
            &PathBuf::from("/home"),
            Some(&PathBuf::from("/home/repo"))
        ),
        Some("…/repo".into())
    );
}

#[test]
fn test_directory_depth() {
    assert_eq!(directory_depth(Path::new("/")), 0);
    assert_eq!(directory_depth(Path::new("/a")), 1);
    assert_eq!(directory_depth(Path::new("/a/b")), 2);
    assert_eq!(directory_depth(Path::new("/a/b/c")), 3);
    assert_eq!(directory_depth(Path::new("a")), 1);
    assert_eq!(directory_depth(Path::new("a/b")), 2);
    assert_eq!(directory_depth(Path::new("a/b/c")), 3);
}
