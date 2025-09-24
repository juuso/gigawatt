use gix::head::Kind;
use gix::progress::Discard;
use gix::state::InProgress;
use gix::status::{Submodule, UntrackedFiles};
use gix::{Repository, discover};
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use crate::utils::current_path;

pub fn repo_state(repo: &Repository) -> Option<&str> {
    match repo.state()? {
        InProgress::ApplyMailbox | InProgress::ApplyMailboxRebase => Some("Applying"),
        InProgress::Bisect => Some("Bisecting"),
        InProgress::CherryPick | InProgress::CherryPickSequence => Some("🍒"),
        InProgress::Merge => Some("Merging"),
        InProgress::Rebase | InProgress::RebaseInteractive => Some("Rebasing"),
        InProgress::Revert | InProgress::RevertSequence => Some("Reverting"),
    }
}

pub fn is_repo_dirty(repo: &Repository) -> bool {
    check_dirty(repo).unwrap_or(false)
}

fn check_dirty(repo: &Repository) -> Option<bool> {
    let had_enough = Arc::new(AtomicBool::new(false));

    let platform = repo
        .status(Discard)
        .ok()?
        .untracked_files(UntrackedFiles::Collapsed)
        .index_worktree_submodules(Submodule::AsConfigured { check_dirty: true })
        .should_interrupt_owned(had_enough.clone());

    let mut it = platform.into_iter(std::iter::empty()).ok()?;
    let dirty = it.next().is_some();
    if dirty {
        had_enough.store(true, Ordering::Relaxed);
    }

    Some(dirty)
}

pub fn repo_head_description(repo: &Repository) -> Option<String> {
    let head = repo.head().ok()?;
    match head.kind {
        Kind::Symbolic(r) => Some(r.name.shorten().to_string()),
        Kind::Detached { target, peeled } => {
            Some(peeled.unwrap_or(target).to_hex_with_len(7).to_string())
        }
        Kind::Unborn(u) => Some(u.shorten().to_string()),
    }
}

pub fn current_repo() -> Option<Repository> {
    let cwd = current_path()?;
    let (p, _) = discover::upwards(&cwd).ok()?;
    let (_, workdir) = p.into_repository_and_work_tree_directories();
    gix::open(workdir?).ok()
}
