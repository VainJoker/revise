// use crate::hooks::types::HookType;
use std::{
    process::{Command, Stdio},
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::error::ReviseResult;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum HookType {
    PreAdd,
    PostAdd,
    PreCommit,
    PostCommit,
    PrePush,
    PostMerge,
}

impl FromStr for HookType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pre-add" => Ok(Self::PreAdd),
            "post-add" => Ok(Self::PostAdd),
            "pre-commit" => Ok(Self::PreCommit),
            "post-commit" => Ok(Self::PostCommit),
            "pre-push" => Ok(Self::PrePush),
            "post-merge" => Ok(Self::PostMerge),
            _ => Err(anyhow::anyhow!("Invalid hook type: {}", s)),
        }
    }
}

pub struct HookRunner;

impl HookRunner {
    pub fn run_hook<F, Args, R>(f: F, args: Args) -> R
    where
        F: FnOnce(Args) -> R,
    {
        f(args)
    }

    pub fn run_command(command: &str) -> ReviseResult<()> {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .arg("/C")
                .arg(command)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()?
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(command)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()?
        };

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "Hook failed: {}\nExit code: {:?}\nStderr: {}",
                command,
                output.status.code(),
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_hook() {
        let func = |(a, b, c): (i32, i32, i32)| -> String {
            println!("running hook");
            println!("a: {a}");
            println!("b: {b}");
            println!("c: {c}");
            format!("result: {}", a + b + c)
        };

        let result = HookRunner::run_hook(func, (3, 4, 5));
        assert_eq!(result, "result: 12");
    }
}
