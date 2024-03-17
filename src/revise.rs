use inquire::InquireError;

use crate::{
    commit::ReviseCommit,
    config::{self, ReviseConfig},
    error::ReviseResult,
    git::GitUtils,
};

pub struct Revise {
    config: ReviseConfig,
    commit: ReviseCommit,
}

impl Default for Revise {
    fn default() -> Self {
        Revise::new()
    }
}
impl Revise {
    pub fn new() -> Self {
        let commit = ReviseCommit::default();
        let config = config::initialize_config().unwrap_or_else(|e| {
            eprintln!("Load config err: {}", e);
            std::process::exit(exitcode::CONFIG);
        });
        Self { config, commit }
    }
    pub fn run(&mut self) -> ReviseResult<()> {
        let result = self.commit.commit(&self.config);

        match result {
            Ok(_) => self.call_git_commit(),
            Err(err) => {
                if let Some(InquireError::OperationCanceled | InquireError::OperationInterrupted) =
                    err.downcast_ref()
                {
                    Ok(())
                } else {
                    Err(err)
                }
            }
        }
    }

    pub fn call_git_commit(&self) -> ReviseResult<()> {
        GitUtils::git_commit(&self.commit.to_string())
    }
}
