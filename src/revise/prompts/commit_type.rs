use anyhow::anyhow;
use inquire::Select;

use crate::{config::ReviseConfig, error::ReviseResult};

pub fn inquire_commit_type(config: &ReviseConfig) -> ReviseResult<String> {
    let msg = "Select the type of change that you're committing:";
    let type_options: Vec<String> = config.get_types();
    let ans = Select::new(msg, type_options.clone()).prompt()?;
    let idx = type_options.iter().position(|s| *s == ans).ok_or_else(|| {
        anyhow!("Error: committing type '{}' not found in the options.", ans)
    })?;
    config
        .get_type_key(idx)
        .ok_or_else(|| anyhow!("Error: no type key found at position {}.", idx))
}
