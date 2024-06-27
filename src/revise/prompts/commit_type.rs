use anyhow::anyhow;
use inquire::Select;

use crate::{config::ReviseConfig, error::ReviseResult};

use super::Inquire;

#[derive(Debug,Clone)]
pub struct Part{
    pub msg: String,
    pub ans: Option<String>,
    pub options: Vec<String>,
}

impl Part {
    pub fn new() -> Self {
        // TODO:
        let options: Vec<String> = Vec::new();
        Self {
            msg: "Select the type of change that you're committing:".to_string(),
            ans: None,
            options
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
        // let options: Vec<String> = self.options;
        // let ans = Select::new(&self.msg, options).prompt()?;
        // let idx = options.iter().position(|s| *s == ans).ok_or_else(|| {
        //     anyhow!("Error: committing type '{}' not found in the options.", ans)
        // })?;
        // TODO:
        // self.ans = Some(config
        //     .get_type_key(idx)
        //     .ok_or_else(|| anyhow!("Error: no type key found at position {}.", idx))?);
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

// pub fn inquire_commit_type(config: &ReviseConfig) -> ReviseResult<String> {
//     let msg = "Select the type of change that you're committing:";
//     let type_options: Vec<String> = config.get_types();
//     let ans = Select::new(msg, type_options.clone()).prompt()?;
//     let idx = type_options.iter().position(|s| *s == ans).ok_or_else(|| {
//         anyhow!("Error: committing type '{}' not found in the options.", ans)
//     })?;
//     config
//         .get_type_key(idx)
//         .ok_or_else(|| anyhow!("Error: no type key found at position {}.", idx))
// }
