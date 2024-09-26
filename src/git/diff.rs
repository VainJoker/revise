pub trait GitDiff {
    fn git_diff(repo: &git2::Repository) -> crate::error::ReviseResult<String> {
        let mut opts = git2::DiffOptions::new(); // 创建一个新的DiffOptions实例

        let head = repo.head()?.peel_to_tree()?; // 获取HEAD指向的tree
        let diff = repo.diff_tree_to_workdir(Some(&head), Some(&mut opts))?; // 获取当前工作目录与HEAD的差异

        let mut content = String::new(); // 预分配内存

        diff.foreach(
            &mut |_, _| true,
            None,
            None,
            Some(&mut |_, _, line| {
                let prefix = match line.origin() {
                    '+' => "+",
                    '-' => "-",
                    _ => " ",
                };
                content.push_str(prefix);
                content.push_str(&String::from_utf8_lossy(line.content()));
                content.push('\n');
                true
            }),
        )?;
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
