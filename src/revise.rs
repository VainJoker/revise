pub mod prompts;
pub mod status;
pub mod template;

use std::collections::HashMap;

use inquire::InquireError;
use status::Status;
use template::Template;

use crate::{
    cli::ReviseCommands,
    config::{self, Hook},
    error::ReviseResult,
    git::GitUtils,
    hook::{HookRunner, HookType},
};

#[derive(Default, Debug)]
pub struct Revise {
    pub template: Template,
    pub status: Status,
    pub message: String,
    pub hooks: HashMap<HookType, Vec<Hook>>,
}

impl Revise {
    pub async fn run(
        &mut self,
        cmd: Option<ReviseCommands>,
    ) -> ReviseResult<()> {
        let cfg = config::get_config();
        self.hooks.clone_from(&cfg.hooks);
        self.run_pre_commit_hooks()?;
        match self.template.run(&cmd).await {
            Ok(msg) => {
                GitUtils::new().commit(&msg)?;
                self.run_post_commit_hooks()?;
                Ok(())
            }
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

    pub fn run_pre_commit_hooks(&self) -> ReviseResult<()> {
        let hooks = self.hooks.get(&HookType::PreCommit);
        let mut sorted_hooks: Vec<_> = hooks.unwrap().iter().collect();
        sorted_hooks.sort_by_key(|h| h.order.unwrap_or(u32::MAX));

        for hook in sorted_hooks {
            if hook.skip.unwrap_or(false) {
                println!("Skipping hook: {}", hook.command);
                continue;
            }
            HookRunner::run_hook(HookRunner::run_command, &hook.command)?;
        }
        Ok(())
    }

    pub fn run_post_commit_hooks(&self) -> ReviseResult<()> {
        let hooks = self.hooks.get(&HookType::PostCommit);
        let mut sorted_hooks: Vec<_> = hooks.unwrap().iter().collect();
        sorted_hooks.sort_by_key(|h| h.order.unwrap_or(u32::MAX));

        for hook in sorted_hooks {
            if hook.skip.unwrap_or(false) {
                println!("Skipping hook: {}", hook.command);
                continue;
            }
            HookRunner::run_hook(HookRunner::run_command, &hook.command)?;
        }
        Ok(())
    }
}
