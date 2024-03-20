use inquire::Editor;

use crate::error::ReviseResult;

pub fn inquire_commit_breaking() -> ReviseResult<Option<String>> {
    let msg = "List any BREAKING CHANGES (optional):";
    let ans = Editor::new(msg)
        .with_formatter(&|submission| {
            let char_count = submission.chars().count();
            if char_count == 0 {
                String::from("<skipped>")
            } else if char_count <= 20 {
                submission.into()
            } else {
                let mut substr: String = submission.chars().take(17).collect();
                substr.push_str("...");
                substr
            }
        })
        .prompt()?;
    if ans.is_empty() {
        Ok(None)
    } else {
        Ok(Some(ans))
    }
}
