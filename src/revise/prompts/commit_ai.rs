use inquire::Select;

use super::Inquire;
use crate::error::ReviseResult;

#[derive(Debug, Clone)]
pub struct Part {
    pub msg: String,
    pub ans: Option<String>,
    pub options: Vec<String>,
}

impl Part {
    pub fn new(options: Vec<String>) -> Self {
        Self {
            msg: "Select the message to be committing :".to_string(),
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
