use crate::utils::{Version, file_name};
use std::env;

pub fn env_shell() -> Option<String> {
    env::var("SHELL").ok().and_then(|s| file_name(&s))
}

pub fn env_term_program() -> Option<String> {
    env::var("TERM_PROGRAM").ok().and_then(|s| file_name(&s))
}

pub fn env_term_version() -> Option<Version> {
    Version::parse(&env::var("TERM_PROGRAM_VERSION").ok()?)
}

pub fn env_home() -> Option<String> {
    env::var("HOME").ok()
}

pub fn supports_truecolor() -> bool {
    env::var("COLORTERM")
        .is_ok_and(|v| v.eq_ignore_ascii_case("truecolor") || v.eq_ignore_ascii_case("24bit"))
}

pub fn supports_256color() -> bool {
    env::var("TERM").is_ok_and(|v| v.to_ascii_lowercase().contains("256color"))
}

pub fn yolo_mode() -> bool {
    env::args().any(|arg| matches!(arg.as_str(), "--yes" | "-y" | "--yolo"))
}
