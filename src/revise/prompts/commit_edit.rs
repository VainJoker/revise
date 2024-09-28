use inquire::{
    Editor,
    ui::{Color, RenderConfig, Styled},
};

use super::Inquire;
use crate::error::ReviseResult;

#[derive(Debug, Clone)]
pub struct Part {
    pub msg: String,
    pub ans: Option<String>,
    pub commit: String,
    pub fg: Color,
}

impl Part {
    pub fn new(commit: String) -> Self {
        Self {
            msg: "You really want to edit this commit manually?".to_string(),
            ans: None,
            commit,
            fg: Color::LightRed,
        }
    }
}

impl Inquire for Part {
    fn inquire(&mut self) -> ReviseResult<()> {
        let ans = Editor::new("")
            .with_predefined_text(&self.commit)
            .with_render_config(RenderConfig::default().with_prompt_prefix(
                Styled::new(self.msg.as_str()).with_fg(self.fg),
            ))
            .prompt()?;
        self.ans = Some(ans);
        Ok(())
    }
}
