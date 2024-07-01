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
    pub fn new() -> Self {
        let cfg = config::get_config();
        let options: Vec<String> = cfg.get_types();
        Self {
            msg: "Select the type of change that you're committing:"
                .to_string(),
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
        let ans = Select::new(&self.msg, self.options.clone()).prompt()?;
        let idx =
            self.options.iter().position(|s| *s == ans).ok_or_else(|| {
                anyhow!(
                    "Error: committing type '{}' not found in the options.",
                    ans
                )
            })?;
        let cfg = config::get_config();
        self.ans = Some(cfg.get_type_key(idx).ok_or_else(|| {
            anyhow!("Error: no type key found at position {}.", idx)
        })?);
        Ok(())
    }
}
