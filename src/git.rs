use std::{path::PathBuf, process::Command};

use colored::Colorize;

use crate::error::ReviseResult;

pub struct GitUtils {}

impl GitUtils {
    pub fn git_repository() -> ReviseResult<PathBuf> {
        let output = Command::new("git")
            .arg("rev-parse")
            .arg("--show-toplevel")
            .output()?;
        if output.status.success() {
            Ok(String::from_utf8(output.stdout)?.trim().into())
        } else {
            anyhow::bail!(
                "Find git repo path failed: {}",
                String::from_utf8(output.stderr)?
            )
        }
    }

    pub fn git_commit(msg: &str) -> ReviseResult<()> {
        let output = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(msg)
            .output()?;
        if output.status.success() {
            let res = format!("{}", "Successfully committed!".green());
            println!("{}", res);
            Ok(())
        } else {
            anyhow::bail!(
                "Git commit failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )
        }
    }
}
