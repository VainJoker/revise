use anyhow::anyhow;
use inquire::Select;

use crate::{config::ReviseConfig, error::ReviseResult};

pub fn inquire_commit_type(config: &ReviseConfig) -> ReviseResult<String> {
    let msg = "Select the type of change that you're committing:";
    let type_options: Vec<String> = config.get_types();
    let ans = Select::new(msg, type_options.clone()).prompt()?;
    let idx = type_options
        .iter()
        .position(|s| *s == ans)
        .ok_or(anyhow!("Error Occurs when select committing type"))?;
    config
        .get_type_key(idx)
        .ok_or(anyhow!("Error Occurs when select committing type"))
}
