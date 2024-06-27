use inquire::{
    ui::{Color, RenderConfig, Styled},
    Editor,
};

use super::Inquire;
use crate::error::ReviseResult;

#[derive(Debug, Clone)]
pub struct Part {
    pub msg: String,
    pub ans: Option<String>,
    pub fg: Color,
}

impl Part {
    pub fn new() -> Self {
        Self {
            msg: "Provide a LONGER description of the change (optional):"
                .to_string(),
            ans: None,
            fg: Color::DarkYellow,
        }
    }
}

impl Default for Part {
    fn default() -> Self {
        Self::new()
    }
}

impl Inquire for Part {
    fn inquire(&mut self) -> ReviseResult<()> {
        let ans = Editor::new(&self.msg)
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
                    Styled::new("<skipped>").with_fg(self.fg),
                ),
            )
            .prompt()?;

        match &*ans {
            "<skipped>" | "" => {}
            _ => {
                self.ans = Some(ans);
            }
        }

        Ok(())
    }
}
