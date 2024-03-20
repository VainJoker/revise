use inquire::Text;

use crate::error::ReviseResult;

pub fn inquire_commit_issue() -> ReviseResult<Option<String>> {
    let msg = "List any ISSUES by this change. E.g.= #31, #34: (optional)";
    let ans = Text::new(msg)
        .with_formatter(&|submission| {
            let char_count = submission.chars().count();
            if char_count == 0 {
                "<skipped>".to_string()
            } else if char_count <= 20 {
                submission.into()
            } else {
                format!("{}...", &submission[..17])
            }
        })
        .prompt()?;
    match &*ans {
        "<skipped>" | "" => Ok(None),
        _ => Ok(Some(ans)),
    }
}
