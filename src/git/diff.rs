pub trait GitDiff {
    fn git_diff(repo: &git2::Repository) -> crate::error::ReviseResult<String> {
        let mut opts = git2::DiffOptions::new(); // 创建一个新的DiffOptions实例

        let head = repo.head()?.peel_to_tree()?; // 获取HEAD指向的tree
        let diff = repo.diff_tree_to_workdir(Some(&head), Some(&mut opts))?; // 获取当前工作目录与HEAD的差异

        let mut content = String::new();
        // 使用DiffFormat::Patch格式打印差异，
        // 但尝试仅提取和传输被修改的行及其上下文
        diff.print(git2::DiffFormat::Patch, |_, _, l| {
            let diff = std::str::from_utf8(l.content()).unwrap();
            content += diff;
            true
        })?;

        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_diff() {
        struct GItDiffImpl;
        impl GitDiff for GItDiffImpl {}

        let _ = GItDiffImpl::git_diff(&git2::Repository::open(".").unwrap())
            .unwrap();
    }
}
