pub mod commit;
pub mod prompts;
pub mod status;
pub mod template;

use std::fmt::Formatter;

use inquire::InquireError;
use status::Status;
use template::Template;

use crate::{
    config, error::ReviseResult,
    utils::git::GitUtils,
};

pub struct Revise{
    pub template: Template,
    // pub status: Status,
    // pub message: String,
}

// impl std::fmt::Display for Revise {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", 1111)
//         // write!(f, "{}", self.message)
//     }
// }


impl Revise {

    pub fn new() -> Self {
        config::initialize_config().unwrap_or_else(|e| {
            eprintln!("Load config err: {e}");
            std::process::exit(exitcode::CONFIG);
        });
        Self { template: Template::default() }
        // let commit = ReviseCommit::default();
        // Self { commit }
    }

    pub fn run(&mut self) -> ReviseResult<()> {
        self.template.run();
        Ok(())
        // let result = self.commit.run();
        //
        // match result {
        //     Ok(()) => self.call_git_commit(),
        //     Err(err) => {
        //         if let Some(
        //             InquireError::OperationCanceled
        //             | InquireError::OperationInterrupted,
        //         ) = err.downcast_ref()
        //         {
        //             Ok(())
        //         } else {
        //             Err(err)
        //         }
        //     }
        // }
    }

    // pub fn call_git_commit(&self) -> ReviseResult<()> {
    //     GitUtils::git_commit(&self.commit.to_string())
    // }
}
