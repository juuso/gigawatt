use crate::gfx::{Rgb24, term_background};
use crate::repo::{current_repo, is_repo_dirty, repo_head_description, repo_state};
use crate::utils::{
    current_path, displayed_path, home_path, supports_256color, supports_truecolor,
};
use std::path::Path;

pub fn print_debug() {
    fn fmt_opt<T: ToString>(opt: Option<T>) -> String {
        opt.map(|o| o.to_string()).unwrap_or_default()
    }
    fn fmt_opt_path(path: Option<&Path>) -> String {
        path.map(|p| p.display().to_string()).unwrap_or_default()
    }

    let repo = current_repo();
    println!("Is in repo: {}", repo.is_some());

    let path = current_path();
    println!("Current path: {}", fmt_opt_path(path.as_deref()));

    let home = home_path();
    println!("Home directory: {}", fmt_opt_path(home.as_deref()));

    let workdir_path = repo.as_ref().and_then(|r| r.workdir());
    println!("Working directory: {}", fmt_opt_path(workdir_path));

    let display_path = displayed_path(workdir_path);
    println!("Path to display: {}", fmt_opt(display_path));

    if let Some(r) = &repo {
        println!("Repo state: {}", repo_state(r).unwrap_or("Clean"));

        let head = repo_head_description(r);
        println!("Repo head: {}", fmt_opt(head));

        println!("Repo dirty: {}", is_repo_dirty(r));
    }

    let bg = term_background()
        .map(Rgb24::from)
        .map(|(r, g, b)| format!("{r}, {g}, {b}"));
    println!("Terminal background: {}", fmt_opt(bg));

    let truecolor = supports_truecolor();
    println!("Terminal supports truecolor: {truecolor}");

    let xterm256color = supports_256color();
    println!("Terminal supports xterm-256color: {xterm256color}");
}
