use add::GitAdd;
use cmit::GitCommit;
use diff::GitDiff;
use repo::GitRepository;

use crate::error::ReviseResult;

pub mod add;
pub mod cmit;
pub mod diff;
pub mod repo;

pub struct GitUtils {
    repo: git2::Repository,
}

impl Default for GitUtils {
    fn default() -> Self {
        Self::new()
    }
}

impl GitUtils {
    pub fn new() -> Self {
        Self {
            repo: Self::git_repo().expect("Failed to get repository"),
        }
    }
    pub fn diff(&self, exclude_files: &[String]) -> ReviseResult<String> {
        Self::git_diff(&self.repo, exclude_files)
    }
    pub fn commit(&self, message: &str) -> ReviseResult<()> {
        Ok(Self::git_cmit(&self.repo, message)?)
    }
    pub fn add(&self, paths: &[String]) -> ReviseResult<()> {
        Self::git_add(&self.repo, paths)
    }
}

impl GitDiff for GitUtils {}
impl GitCommit for GitUtils {}
impl GitRepository for GitUtils {}
impl GitAdd for GitUtils {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_repository() {
        GitUtils::git_repo().unwrap();
    }
}
