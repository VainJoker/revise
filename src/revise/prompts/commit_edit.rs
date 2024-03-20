use inquire::{
    ui::{Color, RenderConfig, Styled},
    Editor,
};

use crate::error::ReviseResult;

pub fn inquire_commit_edit(messages: &str) -> ReviseResult<String> {
    let msg = "You Really want to edit this commit manually?";
    let ans = Editor::new("")
        .with_predefined_text(messages)
        .with_render_config(
            RenderConfig::default()
                .with_prompt_prefix(Styled::new(msg).with_fg(Color::LightRed)),
        )
        .prompt()?;
    Ok(ans)
}
