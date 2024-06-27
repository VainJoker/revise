pub mod commit;
pub mod prompts;
pub mod status;
pub mod template;
pub mod ai;

use inquire::InquireError;
use status::CommitStatus;

use crate::{
    config, error::ReviseResult, revise::commit::ReviseCommit,
    utils::git::GitUtils,
};

// pub struct Revise {
//     // commit: ReviseCommit,
// }
#[derive(Debug, Default)]
pub struct Revise{
    pub template: CommitTemplate,
    pub status: CommitStatus,
    pub message: String,
}

impl std::fmt::Display for ReviseCommit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ReviseCommit {
    pub fn run(&mut self) -> ReviseResult<()> {
        self.template.run()?;
        self.message = self.template.to_string();
        Ok(())
    }
}

impl Default for Revise {
    fn default() -> Self {
        Self::new()
    }
}
impl Revise {

    pub fn new() -> Self {
        config::initialize_config().unwrap_or_else(|e| {
            eprintln!("Load config err: {e}");
            std::process::exit(exitcode::CONFIG);
        });
        let commit = ReviseCommit::default();
        Self { commit }
    }

    pub fn run(&mut self) -> ReviseResult<()> {
        let result = self.commit.run();

        match result {
            Ok(()) => self.call_git_commit(),
            Err(err) => {
                if let Some(
                    InquireError::OperationCanceled
                    | InquireError::OperationInterrupted,
                ) = err.downcast_ref()
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
