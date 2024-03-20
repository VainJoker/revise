use inquire::{
    validator::{ErrorMessage, Validation},
    Text,
};

use crate::error::ReviseResult;

pub fn inquire_commit_subject() -> ReviseResult<String> {
    let msg = "Write a SHORT, IMPERATIVE tense description of the change:\n";
    let ans = Text::new(msg)
        .with_help_message("Infinity more chars allowed")
        .with_validator(|s: &str| {
            if s.is_empty() {
                return Ok(Validation::Invalid(ErrorMessage::Custom(
                    "[ERROR] subject is required".to_string(),
                )));
            }
            Ok(Validation::Valid)
        })
        .prompt()?;
    Ok(ans)
}
