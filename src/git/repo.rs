pub trait GitRepository {
    fn git_repo() -> crate::error::ReviseResult<git2::Repository> {
        Ok(git2::Repository::open_from_env()?)
    }
}
