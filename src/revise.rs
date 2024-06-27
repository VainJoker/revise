pub mod prompts;
pub mod status;
pub mod template;

use inquire::InquireError;
use status::Status;
use template::Template;

use crate::{cli::ReviseCommands, error::ReviseResult, git::GitUtils};

#[derive(Default, Debug)]
pub struct Revise {
    pub template: Template,
    pub status: Status,
    pub message: String,
}

impl Revise {
    pub async fn run(
        &mut self,
        cmd: Option<ReviseCommands>,
    ) -> ReviseResult<()> {
        match self.template.run(&cmd).await {
            Ok(msg) => GitUtils::new().commit(&msg),
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
}
