use add::GitAdd;
use cmit::GitCommit;
use diff::GitDiff;
use repo::GitRepository;

use crate::ReviseResult;

pub mod add;
pub mod cmit;
pub mod diff;
pub mod repo;

pub struct GitUtils;

// pub struct GitUtils {
//     repo: git2::Repository,
// }

// impl Default for GitUtils {
//     fn default() -> Self {
//         Self::new()
//     }
// }

impl Default for GitUtils {
    fn default() -> Self {
        Self::new()
    }
}

impl GitUtils {
    pub const fn new() -> Self {
        Self {}
    }
    pub fn diff(&self, exclude_files: &[String]) -> ReviseResult<String> {
        Self::git_diff(exclude_files)
    }
    pub fn commit(&self, message: &str) -> ReviseResult<()> {
        Self::git_cmit(message)
    }
    pub fn add(&self, paths: &[String]) -> ReviseResult<()> {
        Self::git_add(paths)
    }
}

impl GitDiff for GitUtils {}
impl GitCommit for GitUtils {}
impl GitRepository for GitUtils {}
impl GitAdd for GitUtils {}
