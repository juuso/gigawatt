use crate::gfx::green;
use crate::utils::env_shell;
use std::env;

const SUPPORTED_SHELLS: &[&str] = &["bash", "fish", "zsh"];

pub enum Invocation {
    Shell { name: &'static str, shell: String },
    Bare { name: &'static str },
    Unknown { name: String },
}

#[derive(Clone, Copy)]
struct Command {
    name: &'static str,
    aliases: &'static [&'static str],
    description: &'static str,
    wants_shell: bool,
}

impl Command {
    const fn new(name: &'static str, description: &'static str, wants_shell: bool) -> Self {
        Self {
            name,
            aliases: &[],
            description,
            wants_shell,
        }
    }

    const fn with_aliases(mut self, aliases: &'static [&'static str]) -> Self {
        self.aliases = aliases;
        self
    }
}

const COMMANDS: &[Command] = &[
    Command::new("", "Prompt for the shell", true),
    Command::new("init", "Shell script to initialize the prompt", true),
    Command::new("guide", "Instructions for the shell", true),
    Command::new("please", "Add the init line to the shell rc file", true),
    Command::new("debug", "Debugging info", false),
    Command::new("help", "Usage info", false).with_aliases(&["-h", "--help"]),
    Command::new("version", "Version info", false).with_aliases(&["-v", "--version"]),
];

pub fn parse_args() -> Invocation {
    let mut args = env::args().skip(1);
    let command_name = args.next().unwrap_or_default();

    if command_name.is_empty() {
        let shell = String::new();
        return Invocation::Shell { name: "", shell };
    }

    if let Some(shell) = find_shell_by_name(&command_name) {
        return Invocation::Shell { name: "", shell };
    }

    if let Some(command) = find_command_by_name(&command_name) {
        let name = command.name;
        if command.wants_shell {
            let shell = args.next().unwrap_or_else(default_shell);
            Invocation::Shell { name, shell }
        } else {
            Invocation::Bare { name }
        }
    } else {
        Invocation::Unknown { name: command_name }
    }
}

fn find_command_by_name(name: &str) -> Option<&'static Command> {
    COMMANDS.iter().find(|c| {
        c.name.eq_ignore_ascii_case(name) || c.aliases.iter().any(|a| a.eq_ignore_ascii_case(name))
    })
}

fn find_shell_by_name(name: &str) -> Option<String> {
    SUPPORTED_SHELLS
        .iter()
        .find(|s| s.eq_ignore_ascii_case(name))
        .map(|s| s.to_string())
}

fn default_shell() -> String {
    if let Some(shell) = env_shell() {
        shell
    } else {
        eprintln!("🆘 Could not determine your shell");
        std::process::exit(1);
    }
}

fn command_syntax(command: &Command) -> String {
    let pkg = env!("CARGO_PKG_NAME");

    let full_name = if command.name.is_empty() {
        pkg.to_string()
    } else {
        format!("{pkg} {}", command.name)
    };

    let names = if command.aliases.is_empty() {
        full_name
    } else {
        let aliases = command.aliases.join(", ");
        format!("{full_name}, {aliases}")
    };

    if command.wants_shell {
        let shell_arg = format!("[{}]", SUPPORTED_SHELLS.join("|"));
        format!("{names} {shell_arg}")
    } else {
        names
    }
}

pub fn print_usage() {
    let syntaxes: Vec<_> = COMMANDS.iter().map(command_syntax).collect();
    let syntax_column_width = syntaxes.iter().map(|s| s.len()).max().unwrap_or(0);

    for (command, syntax) in COMMANDS.iter().zip(syntaxes.iter()) {
        let padding = " ".repeat(syntax_column_width - syntax.len());
        println!("{}{padding}  {}", green(syntax), command.description);
    }
}
