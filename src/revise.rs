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
    pub action: Option<Action>
}

pub enum Action {
    Translate,
    Generate
}

impl Revise {

    pub async fn run(&mut self) -> ReviseResult<()> {
        self.action = Some(Action::Translate);
        match &self.action {
            Some(action) => {
                self.template.run_action(action).await?;
                self.message = self.template.to_string();
                let mut confirm = commit_confirm::Part::new(self.template.clone());
                confirm.inquire()?;
            }
            None => {
                self.template.run_default()?;
                self.message = self.template.to_string();
                let mut confirm = commit_confirm::Part::new(self.template.clone());
                confirm.inquire()?;
            }
        }
        Ok(())
    }

}


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
