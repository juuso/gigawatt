use crate::gfx::{
    NonPrintingWrappers, Oklab, Srgb, TermColor, TextBuilder, prompt_256color_bg_colors,
    prompt_oklab_bg_colors, term_background,
};
use crate::repo::{current_repo, is_repo_dirty, repo_head_description, repo_state};
use crate::utils::{displayed_path, supports_256color, supports_truecolor};

const BASH_WRAPPERS: NonPrintingWrappers = ("\\[", "\\]");
const ZSH_WRAPPERS: NonPrintingWrappers = ("%{", "%}");

pub fn print_prompt(shell: &str) {
    let term_bg = term_background().unwrap_or(Srgb::WHITE);
    let (path_bg, git_bg) = prompt_bg_colors(term_bg);

    let repo = current_repo();
    let workdir = repo.as_ref().and_then(|r| r.workdir());

    let is_dark = Oklab::from(term_bg).is_dark();

    let green = if is_dark {
        Some(TermColor::Palette(10))
    } else {
        Some(TermColor::Palette(2))
    };

    let mut b = TextBuilder::new();
    b.fg(Some(path_bg)).text("\n\u{e0b6}");

    let path = displayed_path(workdir).unwrap_or("🆘".into());
    b.fg(None).bg(Some(path_bg)).text(&format!(" {path} "));

    b.fg(Some(path_bg)).bg(Some(git_bg)).text("\u{e0b0}");

    if let Some(r) = &repo
        && let Some(head) = repo_head_description(r)
    {
        b.fg(None).text(" 🌵 ");

        if let Some(s) = repo_state(r) {
            b.bold(is_dark).fg(green).text(&format!("{s} ")).bold(false);
        }

        b.fg(None).text(&format!("{head} "));

        if is_repo_dirty(r) {
            b.bold(is_dark).fg(green).text("! ").bold(false);
        }
    }

    b.fg(Some(git_bg)).bg(None).text("\u{e0b0}\n");
    b.bold(true).fg(green).text("\u{276f} ");

    let prompt = b.build().render(wrappers_for_shell(shell));
    print!("{prompt}");
}

fn prompt_bg_colors(term_bg: Srgb) -> (TermColor, TermColor) {
    let is_dark = Oklab::from(term_bg).is_dark();

    if supports_truecolor() {
        let (a, b) = prompt_oklab_bg_colors(term_bg);
        (TermColor::from(a), TermColor::from(b))
    } else if supports_256color() {
        let (a, b) = prompt_256color_bg_colors(term_bg);
        (TermColor::Palette(a), TermColor::Palette(b))
    } else if is_dark {
        (TermColor::Palette(8), TermColor::Palette(0))
    } else {
        (TermColor::Palette(7), TermColor::Palette(15))
    }
}

fn wrappers_for_shell(shell: &str) -> Option<NonPrintingWrappers> {
    match shell {
        "bash" => Some(BASH_WRAPPERS),
        "zsh" => Some(ZSH_WRAPPERS),
        _ => None,
    }
}
