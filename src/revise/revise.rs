use std::fmt::Formatter;

use anyhow::anyhow;
use clap::Parser;
use inquire::InquireError;

use crate::{
    config,
    error::ReviseResult,
    revise::{
        args::{ReviseArgs, ReviseCommand::Commit},
        commit::ReviseCommit,
    },
    utils::git::GitUtils,
};

#[derive(Debug)]
pub struct Revise {
    args: ReviseArgs,
    commit: ReviseCommit,
}

impl std::fmt::Display for Revise {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.commit.message)
    }
}

impl Default for Revise {
    fn default() -> Self {
        Self::new()
    }
}

impl Revise {
    pub fn new() -> Self {
        let args = ReviseArgs::parse();
        config::initialize_config().unwrap_or_else(|e| {
            eprintln!("Load config err: {e}");
            std::process::exit(exitcode::CONFIG);
        });
        let commit = ReviseCommit::default();
        Self { args, commit }
    }

    pub fn run(&mut self) -> ReviseResult<()> {
        eprintln!("{self:#?}");
        let res = match (self.args.message.take(), self.args.command.take()) {
            (Some(_), Some(_)) => {
                Err(anyhow!("You should not use -m with commit"))
            }
            (Some(msg), None) => {
                self.commit.message = msg;
                self.call_git_commit()
            }
            (None, Some(Commit(ct))) => {
                self.commit.message = ct.to_string();
                self.call_git_commit()
            }
            (None, None) => {
                self.commit.run()?;
                self.call_git_commit()
            }
        };

        if let Err(err) = &res {
            if let Some(
                InquireError::OperationCanceled
                | InquireError::OperationInterrupted,
            ) = err.downcast_ref()
            {
                return Ok(());
            }
        };

        res
    }

    pub fn call_git_commit(&self) -> ReviseResult<()> {
        GitUtils::git_commit(&self.to_string())
    }
}
