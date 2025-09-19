use git2::{ErrorCode, ObjectType, Reference, Repository, RepositoryState, StatusOptions};

use crate::utils::current_path;

pub fn repo_state(repo: &Repository) -> Option<&str> {
    match repo.state() {
        RepositoryState::Clean => None,
        RepositoryState::ApplyMailbox | RepositoryState::ApplyMailboxOrRebase => Some("Applying"),
        RepositoryState::Bisect => Some("Bisecting"),
        RepositoryState::CherryPick | RepositoryState::CherryPickSequence => Some("🍒"),
        RepositoryState::Merge => Some("Merging"),
        RepositoryState::Rebase
        | RepositoryState::RebaseInteractive
        | RepositoryState::RebaseMerge => Some("Rebasing"),
        RepositoryState::Revert | RepositoryState::RevertSequence => Some("Reverting"),
    }
}

pub fn is_repo_dirty(repo: &Repository) -> bool {
    let mut opts = StatusOptions::new();
    opts.include_untracked(true).include_ignored(false);
    match repo.statuses(Some(&mut opts)) {
        Ok(statuses) => !statuses.is_empty(),
        Err(_) => false,
    }
}

pub fn repo_head_description(repo: &Repository) -> Option<String> {
    match repo.head() {
        Ok(r) => {
            if r.is_branch() {
                Some(r.shorthand()?.to_string())
            } else {
                commit_description(&r)
            }
        }
        Err(e) if e.code() == ErrorCode::UnbornBranch => repo_unborn_head_description(repo),
        _ => None,
    }
}

fn commit_description(r: &Reference) -> Option<String> {
    let obj = r.peel(ObjectType::Commit).ok()?;
    Some(obj.short_id().ok()?.as_str()?.to_string())
}

fn repo_unborn_head_description(repo: &Repository) -> Option<String> {
    match repo.find_reference("HEAD") {
        Ok(r) => {
            let target = r.symbolic_target()?;
            let commit = target.strip_prefix("refs/heads/")?;
            Some(commit.to_string())
        }
        Err(_) => None,
    }
}

pub fn current_repo() -> Option<Repository> {
    let cwd = current_path()?;
    Repository::discover(&cwd).ok()
}
