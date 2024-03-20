use inquire::{
    ui::{Color, RenderConfig, Styled},
    Editor,
};

use crate::error::ReviseResult;

pub fn inquire_commit_breaking() -> ReviseResult<Option<String>> {
    let msg = "List any BREAKING CHANGES (optional):";
    let ans = Editor::new(msg)
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
        .with_render_config(
            RenderConfig::default().with_canceled_prompt_indicator(
                Styled::new("<skipped>").with_fg(Color::DarkYellow),
            ),
        )
        .prompt()?;
    match &*ans {
        "<skipped>" | "" => Ok(None),
        _ => Ok(Some(ans)),
    }
}
