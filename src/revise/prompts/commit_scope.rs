use inquire::{Select, Text};

use crate::{config::ReviseConfig, error::ReviseResult};

pub fn inquire_commit_scope(
    config: &ReviseConfig,
) -> ReviseResult<Option<String>> {
    let msg = "Denote the SCOPE of this change (optional):";
    let mut scope_options: Vec<String> = config.get_scopes();

    // Prepend "empty" if not present
    if !scope_options.contains(&"empty".to_string()) {
        scope_options.insert(0, "empty".to_string());
    }

    // Append "custom" if not present
    if !scope_options.contains(&"custom".to_string()) {
        scope_options.push("custom".to_string());
    }

    let ans = Select::new(msg, scope_options).prompt()?;

    if ans == "custom" {
        let ans = Text::new("Denote the SCOPE of this change:").prompt()?;
        Ok(Some(ans).filter(|a| !a.is_empty()))
    } else if ans == "empty" {
        Ok(None)
    } else {
        Ok(Some(ans))
    }
}
