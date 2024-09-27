pub trait GitCommit {
    fn git_cmit(
        repo: &git2::Repository,
        message: &str,
    ) -> Result<(), git2::Error> {
        // 写入index
        let mut index = repo.index()?;
        let oid = index.write_tree()?;

        let tree = repo.find_tree(oid)?;

        // 获取当前HEAD的commit
        let head = repo.head().ok();
        let parent_commit = head.as_ref().and_then(|h| h.peel_to_commit().ok());

        let parents = parent_commit.iter().collect::<Vec<_>>();

        let conf = git2::Config::open_default()?;
        let name = conf.get_string("user.name")?;
        let email = conf.get_string("user.email")?;
        let signature = git2::Signature::now(&name, &email)?;

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &parents,
        )?;

        Ok(())
    }
}
