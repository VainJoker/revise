pub mod cmit;
pub mod diff;
pub mod repo;
pub mod conf;

use std::{path::PathBuf, process::Command};

use colored::Colorize;
use git2::{DiffOptions, Repository};

use crate::error::ReviseResult;

pub struct GitUtils {}


impl GitUtils {
    pub fn git_repository() -> ReviseResult<PathBuf> {
        // Ok(Repository::open_from_env()?)
        // let current_path = std::env::current_dir()?;
        // let repository =
        //     git2::Repository::discover(current_path)?;
        // let repository = git2::Repository::open_from_env()?;
        Ok(PathBuf::new())
        // Ok(repository.path().parent().unwrap().to_path_buf())
    }

    // pub fn git_commit(msg: &str) -> ReviseResult<()> {
    //     let r = git2::Repository::open_from_env().unwrap();
    //     eprintln!("{:#?}",r.path());
    //     r.commit(update_ref, author, committer, message, tree, parents);
    //     Ok(())
    // }

    fn git_commit(repo: &git2::Repository) -> ReviseResult<()> {
        let mut index = repo.index().unwrap();
        let oid = index.write_tree().unwrap();
        let signature = repo.signature().unwrap();
        let parent_commit = repo.head().unwrap().peel_to_commit().unwrap();
        let tree = repo.find_tree(oid).unwrap();
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            "added some file",
            &tree,
            &[&parent_commit],
        )
            .unwrap();
        Ok(())
    }

    fn git_diff(repo: &git2::Repository) -> ReviseResult<()> {
        let mut opts = DiffOptions::new();
        Ok(())
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_repository() {
        GitUtils::git_repository().unwrap();
    }
}
