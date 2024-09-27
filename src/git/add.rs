use git2::{IndexAddOption, Repository};

use crate::ReviseResult;

pub trait GitAdd {
    fn git_add(repo: &Repository, paths: &[String]) -> ReviseResult<()> {
        let mut index = repo.index()?;
        index.add_all(paths.iter(), IndexAddOption::DEFAULT, None)?;

        index.write()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct TestGitAdd {}

    impl GitAdd for TestGitAdd {}

    #[ignore]
    #[test]
    fn test_git_add() {
        let repo = Repository::open(".").unwrap();
        let paths = vec![".".to_string()];
        let result = TestGitAdd::git_add(&repo, &paths);
        assert!(result.is_ok());
    }
}
