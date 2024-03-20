use std::fmt::Formatter;

use crate::{
    error::ReviseResult,
    revise::{status::CommitStatus, template::CommitTemplate},
};

#[derive(Debug, Default)]
pub struct ReviseCommit {
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
    pub fn new() -> ReviseResult<Self> {
        let template = CommitTemplate::new()?;
        Ok(Self {
            template: template.clone(),
            status: CommitStatus::default(),
            message: template.to_string(),
        })
    }

    pub fn run(&mut self) -> ReviseResult<()> {
        self.template.run()?;
        self.message = self.template.to_string();
        Ok(())
    }
}
