use inquire::{
    ui::{Color, RenderConfig, Styled},
    Editor,
};

use crate::error::ReviseResult;

pub fn inquire_commit_body() -> ReviseResult<Option<String>> {
    let msg = "Provide a LONGER description of the change (optional):";
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
        .with_render_config(
            RenderConfig::default().with_canceled_prompt_indicator(
                Styled::new("<skipped>").with_fg(Color::DarkYellow),
            ),
        )
        .prompt()?;
    if ans.is_empty() {
        Ok(None)
    } else {
        Ok(Some(ans))
    }
}
