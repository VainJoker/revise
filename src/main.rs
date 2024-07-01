// use git_revise::{config, revise::Revise};
// use human_panic::setup_panic;
//
// fn main() {
//     setup_panic!();
//     config::initialize_config().unwrap_or_else(|e| {
//         eprintln!("Load config err: {e}");
//         std::process::exit(exitcode::CONFIG);
//     });
//     Revise::default().run().unwrap();
//     // match Revise::default().run() {
//     //     Ok(()) => {
//     //         std::process::exit(exitcode::OK);
//     //     }
//     //     Err(e) => {
//     //         eprintln!("Error occurred when trying to commit, err: {e}");
//     //         std::process::exit(exitcode::DATAERR);
//     //     }
//     // }
// }

use git2::{DiffOptions, Error, Repository, DiffFormat};
use std::str;

fn print_diff_line(line: git2::DiffLine) -> bool {
    let content = str::from_utf8(line.content()).unwrap();
    match line.origin() {
        '+' | '-' => print!("{}", content), // 仅打印添加或删除的行
        ' ' => print!(" {}", content), // 打印上下文行，但可以考虑限制数量
        _ => (),
    }
    true
}

fn run() -> Result<(), Error> {
    let repo = Repository::open(".")?; // 打开当前目录的git仓库

    let mut opts = DiffOptions::new(); // 创建一个新的DiffOptions实例

    let head = repo.head()?.peel_to_tree()?; // 获取HEAD指向的tree
    let diff = repo.diff_tree_to_workdir(Some(&head), Some(&mut opts))?; // 获取当前工作目录与HEAD的差异

    // 使用DiffFormat::Patch格式打印差异，但尝试仅提取和传输被修改的行及其上下文
    diff.print(DiffFormat::Patch, |_, _, l| print_diff_line(l))?;
    Ok(())
}

fn main() {
    match run() {
        Ok(()) => {}
        Err(e) => println!("error: {}", e),
    }
}

