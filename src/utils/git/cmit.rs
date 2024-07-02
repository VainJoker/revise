pub trait GitCommit {
    fn git_cmit() -> crate::error::ReviseResult<()>{
        Ok(())
    }
}

// fn create_initial_commit(repo: &git2::Repository) {
//     let signature = repo.signature().unwrap();
//     let oid = repo.index().unwrap().write_tree().unwrap();
//     let tree = repo.find_tree(oid).unwrap();
//     repo.commit(
//         Some("HEAD"),
//         &signature,
//         &signature,
//         "Initial commit",
//         &tree,
//         &[],
//     )
//     .unwrap();
// }
//
// fn commit(repo: &git2::Repository,message: &str) {
//     let mut index = repo.index().unwrap();
//     let oid = index.write_tree().unwrap();
//     let signature = repo.signature().unwrap();
//     let parent_commit = repo.head().unwrap().peel_to_commit().unwrap();
//     let tree = repo.find_tree(oid).unwrap();
//     repo.commit(
//         Some("HEAD"),
//         &signature,
//         &signature,
//         message,
//         &tree,
//         &[&parent_commit],
//     )
//     .unwrap();
// }

fn git_commit(repo: &git2::Repository, message: &str) -> Result<(), git2::Error> {
    // let repo = Repository::open(repo_path)?;

    // 获取index
    // let mut index = repo.index()?;
    //
    // // 添加所有更改到index
    // index.add_all(["."].iter(), git2::IndexAddOption::DEFAULT, None)?;

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
    eprintln!("{:#?},{:#?}",name,email);
    let signature = git2::Signature::now(&name, &email)?;

    // 创建commit
    repo.commit(Some("HEAD"), &signature, &signature, message, &tree, &parents)?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_git_commit() {
        git_commit(&git2::Repository::open(".").unwrap(), "Test").unwrap();
    }
    
}
