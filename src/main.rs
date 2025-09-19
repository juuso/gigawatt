mod command;
mod gfx;
mod help;
mod init;
mod prompt;
mod repo;
mod utils;

use crate::command::{
    Invocation::{Bare, Shell, Unknown},
    parse_args, print_usage,
};
use crate::help::{please, print_debug, print_guide};
use crate::init::print_init;
use crate::prompt::print_prompt;
use std::env;

fn main() {
    match parse_args() {
        Shell { name, shell } => match name {
            "" => print_prompt(&shell),
            "init" => print_init(&shell),
            "guide" => print_guide(&shell),
            "please" => please(&shell),
            _ => unknown_command(name),
        },
        Bare { name } => match name {
            "help" => print_usage(),
            "version" => print_version(),
            "debug" => print_debug(),
            _ => unknown_command(name),
        },
        Unknown { name } => unknown_command(&name),
    }
}

pub fn unknown_command(unknown: &str) -> ! {
    let pkg = env!("CARGO_PKG_NAME");
    eprintln!("🆘 Unknown command '{pkg} {unknown}'. Maybe try '{pkg} help'?");
    print_usage();
    std::process::exit(1);
}

fn print_version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}
