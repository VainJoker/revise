use inquire::{
    validator::{ErrorMessage, Validation},
    Text,
};

use super::Inquire;
use crate::error::ReviseResult;

#[derive(Debug, Clone)]
pub struct Part {
    pub msg: String,
    pub ans: Option<String>,
}

impl Part {
    pub fn new() -> Self {
        Self {
            msg: "Write a SHORT, IMPERATIVE tense description of the change:\n"
                .to_string(),
            ans: None,
        }
    }
}

impl Default for Part {
    fn default() -> Self {
        Self::new()
    }
}

impl Inquire for Part {
    fn inquire(&mut self) -> ReviseResult<()> {
        let ans = Text::new(&self.msg)
            .with_help_message("Infinity more chars allowed")
            .with_validator(|s: &str| {
                if s.is_empty() {
                    return Ok(Validation::Invalid(ErrorMessage::Custom(
                        "[ERROR] Subject is required and cannot be empty"
                            .to_string(),
                    )));
                }
                Ok(Validation::Valid)
            })
            .prompt()?;
        self.ans = Some(ans);
        Ok(())
    }
}
