pub mod prompts;
pub mod status;
pub mod template;

use prompts::{commit_confirm, Inquire};
use status::Status;
use template::Template;

use crate::error::ReviseResult;

#[derive(Default)]
pub struct Revise {
    pub template: Template,
    pub status: Status,
    pub message: String,
}

// impl std::fmt::Display for Revise {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", 1111)
//         // write!(f, "{}", self.message)
//     }
// }
//

impl Revise {
    // pub fn new() -> Self {
    //     config::initialize_config().unwrap_or_else(|e| {
    //         eprintln!("Load config err: {e}");
    //         std::process::exit(exitcode::CONFIG);
    //     });
    //     Self { template: Template::default(), status: Status::default(),
    // message: String::new() }     // let commit = ReviseCommit::default();
    //     // Self { commit }
    // }

    pub fn run(&mut self) -> ReviseResult<()> {
        self.template.run()?;
        self.message = self.template.to_string();
        let mut confirm = commit_confirm::Part::new(self.template.clone());
        confirm.inquire().unwrap();
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
