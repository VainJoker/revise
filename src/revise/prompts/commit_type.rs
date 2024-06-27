use anyhow::anyhow;
use inquire::Select;

use crate::{config::ReviseConfig, error::ReviseResult};

use super::Inquire;

pub struct CType{
    pub msg: String,
    pub ans: Option<String>,
    pub options: Vec<String>,
}

impl CType {
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

impl Default for CType {
    fn default() -> Self {
        Self::new()
    }
}

impl Inquire for CType {
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
