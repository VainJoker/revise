use cmit::GitCommit;
use diff::GitDiff;
use repo::GitRepository;

use crate::error::ReviseResult;

pub mod cmit;
pub mod diff;
pub mod repo;

pub struct GitUtils{
    repo: git2::Repository
}

impl GitUtils {
    pub fn new() ->Self{
        Self { repo: GitUtils::git_repo().expect("Failed to get repository") }
    }
    pub fn diff(&self) -> ReviseResult<String>{
        GitUtils::git_diff(&self.repo)
    }
    pub fn commit(&self,message:&str) -> ReviseResult<()>{
        Ok(GitUtils::git_cmit(&self.repo,message)?)
    }
}


impl GitDiff for GitUtils {}
impl GitCommit for GitUtils {}
impl GitRepository for GitUtils{}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_repository() {
        GitUtils::git_repo().unwrap();
    }
}
