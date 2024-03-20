use inquire::{Select, Text};

use crate::{config::ReviseConfig, error::ReviseResult};

pub fn inquire_commit_scope(
    config: &ReviseConfig,
) -> ReviseResult<Option<String>> {
    let msg = "Denote the SCOPE of this change (optional):";
    let mut scope_options: Vec<String> = config.get_scopes();
    scope_options.push("empty".to_string());
    scope_options.push("custom".to_string());
    let ans = Select::new(msg, scope_options).prompt()?;
    match ans.as_str() {
        "custom" => {
            let msg = "Denote the SCOPE of this change:";
            let ans = Text::new(msg).prompt()?;
            Ok(Some(ans).filter(|a| !a.is_empty()))
        }
        "empty" => Ok(None),
        _ => Ok(Some(ans)),
    }
}
