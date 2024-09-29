// pub trait GitRepository {
//     fn git_repo() -> crate::error::ReviseResult<git2::Repository> {
//         Ok(git2::Repository::open_from_env()?)
//     }
// }
use std::process::Command;

use crate::error::ReviseResult;

pub trait GitRepository {
    fn git_repo() -> ReviseResult<String> {
        let output = Command::new("git")
            .args(["rev-parse", "--show-toplevel"])
            .output()?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "Failed to get git repository: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_git_repo() {
        struct GitRepoImpl;
        impl GitRepository for GitRepoImpl {}
        let repo = GitRepoImpl::git_repo().unwrap();
        println!("Git repository: {repo}");
    }
}
