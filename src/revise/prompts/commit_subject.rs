use inquire::{
    validator::{ErrorMessage, Validation},
    Text,
};

use crate::error::ReviseResult;

use super::Inquire;

#[derive(Debug,Clone)]
pub struct Part{
    pub msg: String,
    pub ans: Option<String>,
}

impl Part {
    pub fn new() -> Self {
        Self {
            msg: "Write a SHORT, IMPERATIVE tense description of the change:\n".to_string(),
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
impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ans = self.ans.clone();
        let res = match &ans {
            Some(s) => s,
            None => ""
        };
        write!(f, "{res}")
    }
}

// pub fn inquire_commit_subject() -> ReviseResult<String> {
//     let ans = Text::new(msg)
//         .with_help_message("Infinity more chars allowed")
//         .with_validator(|s: &str| {
//             if s.is_empty() {
//                 return Ok(Validation::Invalid(ErrorMessage::Custom(
//                     "[ERROR] Subject is required and cannot be empty"
//                         .to_string(),
//                 )));
//             }
//             Ok(Validation::Valid)
//         })
//         .prompt()?;
//     Ok(ans)
// }
