use anyhow::anyhow;
use inquire::Select;

use super::Inquire;
use crate::{
    config::{self},
    error::ReviseResult,
};

#[derive(Debug, Clone)]
pub struct Part {
    pub msg: String,
    pub ans: Option<String>,
    pub options: Vec<String>,
}

impl Part {
    pub fn new(options: Vec<String>) -> Self {
        Self {
            msg: "Select the message that you want to use and you can edit manully later:"
                .to_string(),
            ans: None,
            options,
        }
    }
}

impl Inquire for Part {
    fn inquire(&mut self) -> ReviseResult<()> {
        let ans = Select::new(&self.msg, self.options.clone()).prompt()?;
        self.ans = Some(ans);
        Ok(())
    }
}
