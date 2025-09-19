// Not every supported shell is guaranteed to have a convenience initializer.
// For some shells, there can just be manual installation instructions.
pub fn print_init(shell: &str) {
    let pkg = env!("CARGO_PKG_NAME");

    match shell {
        "bash" => {
            println!("PROMPT_COMMAND='PS1=\"$(command {pkg} bash)\"'");
        }
        "zsh" => {
            println!("gigawatt_precmd() {{ PROMPT=\"$(command {pkg} zsh)\"; }}");
            println!("autoload -Uz add-zsh-hook");
            println!("add-zsh-hook precmd gigawatt_precmd");
        }
        "fish" => {
            println!("function fish_prompt; command {pkg} fish; end");
        }
        _ => {}
    }
}
