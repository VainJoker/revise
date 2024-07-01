use inquire::{Select, Text};

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
    pub fn new() -> Self {
        let cfg = config::get_config();
        let mut options: Vec<String> = cfg.get_scopes();
        // Prepend "empty" if not present
        if !options.contains(&"empty".to_string()) {
            options.insert(0, "empty".to_string());
        }

        // Append "custom" if not present
        if !options.contains(&"custom".to_string()) {
            options.push("custom".to_string());
        }

        Self {
            msg: "Denote the SCOPE of this change (optional):".to_string(),
            ans: None,
            options,
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
        let mut ans = Select::new(&self.msg, self.options.clone()).prompt()?;

        if ans == "custom" {
            ans = Text::new("Denote the SCOPE of this change:").prompt()?;
            self.ans = Some(ans).filter(|a| !a.is_empty());
        } else if ans == "empty" {
            self.ans = None;
        } else {
            self.ans = Some(ans);
        };
        Ok(())
    }
}
