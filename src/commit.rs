use std::fmt::{format, Formatter};
use std::ops::Add;
use anyhow::anyhow;
use inquire::{
    validator::{ErrorMessage, Validation},
    Editor, Select, Text,
};

use crate::{config::ReviseConfig, error::ReviseResult};

#[derive(Debug, Default)]
pub struct ReviseCommit {
    pub commit_type: String,
    pub commit_scope: Option<String>,
    pub commit_subject: String,
    pub commit_body: Option<String>,
    pub commit_breaking: Option<String>,
    pub commit_issue: Option<String>,
    pub commit_confirm_commit: String,
}

impl std::fmt::Display for ReviseCommit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::new();
        msg.push_str(&self.commit_type);
        match &self.commit_scope {
            None => {}
            Some(scope) => {
                let scope = format!("({})",scope);
                msg.push_str(&scope);
            }
        }
        write!(f, "{}", msg)
    }
}

impl ReviseCommit {
    pub fn commit(&mut self, config: &ReviseConfig) -> ReviseResult<&Self> {
        self.inquire_commit_type(config)?;
        self.inquire_commit_scope(config)?;
        self.inquire_commit_subject()?;
        self.inquire_commit_body()?;
        self.inquire_commit_breaking()?;
        self.inquire_commit_issue()?;
        self.inquire_confirm_commit()?;
        Ok(self)
    }

    pub fn inquire_commit_type(&mut self, config: &ReviseConfig) -> ReviseResult<()> {
        let msg = "Select the type of change that you're committing:";
        let type_options: Vec<String> = config.get_types();
        let ans = Select::new(msg, type_options.clone()).prompt()?;
        let idx = type_options
            .iter()
            .position(|s| *s == ans)
            .ok_or(anyhow!("Error Occurs when select committing type"))?;
        self.commit_type = config
            .get_type_key(idx)
            .ok_or(anyhow!("Error Occurs when select committing type"))?;
        Ok(())
    }

    pub fn inquire_commit_scope(&mut self, config: &ReviseConfig) -> ReviseResult<()> {
        let msg = "Denote the SCOPE of this change (optional):";
        let mut scope_options: Vec<String> = config.get_scopes();
        scope_options.push("empty".to_string());
        scope_options.push("custom".to_string());
        let ans = Select::new(msg, scope_options).prompt()?;
        self.commit_scope = match ans.as_str() {
            "custom" => {
                let msg = "Denote the SCOPE of this change:";
                let ans = Text::new(msg).prompt()?;
                Some(ans).filter(|a| !a.is_empty())
            },
            "empty" => None,
            _ => Some(ans),
        };
        Ok(())
    }

    pub fn inquire_commit_subject(&mut self) -> ReviseResult<()> {
        let msg = "Write a SHORT, IMPERATIVE tense description of the change:";
        let ans = Text::new(msg)
            .with_validator(|s: &str| {
                if s.is_empty() {
                    return Ok(Validation::Invalid(ErrorMessage::Custom(
                        "[ERROR] subject is required".to_string(),
                    )));
                }
                Ok(Validation::Valid)
            })
            .prompt()?;
        self.commit_subject = ans;
        Ok(())
    }

    pub fn inquire_commit_body(&mut self) -> ReviseResult<()> {
        let msg = "Provide a LONGER description of the change (optional):";
        let description = Editor::new(msg).prompt()?;
        if description.is_empty() {
            self.commit_body = None
        } else {
            self.commit_body = Some(description)
        }
        Ok(())
    }

    pub fn inquire_commit_breaking(&mut self) -> ReviseResult<()> {
        let msg = "List any BREAKING CHANGES (optional):";
        let ans = Text::new(msg).prompt()?;
        if ans.is_empty() {
            self.commit_breaking = None
        } else {
            self.commit_breaking = Some(ans)
        }
        Ok(())
    }

    pub fn inquire_commit_issue(&mut self) -> ReviseResult<()> {
        let msg = "List any ISSUES by this change. E.g.= #31, #34:";
        let ans = Text::new(msg).prompt()?;
        if ans.is_empty() {
            self.commit_issue = None
        } else {
            self.commit_issue = Some(ans)
        }
        Ok(())
    }

    pub fn inquire_confirm_commit(&mut self) -> ReviseResult<()> {
        // let commit_message = String::new();
        // format!("{} {:?}", self.commit_type, self.commit_scope);
        // commit_message.add(&self.commit_type);
        // if self.commit {  }
        // eprintln!("{}",commit_message.clone());
        let msg = "Are you sure you want to proceed with the commit above?";
        let _ans = Text::new(msg).prompt()?;
        Ok(())
    }
}
