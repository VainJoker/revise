use inquire::{
    ui::{Color, RenderConfig, Styled},
    Editor,
};

use crate::error::ReviseResult;

use super::Inquire;

#[derive(Debug,Clone)]
pub struct Part{
    pub msg: String,
    pub ans: Option<String>,
    pub commit: String,
    pub fg: Color
}

impl Part {
    pub fn new() -> Self {
        Self {
            msg: "You Really want to edit this commit manually?".to_string(),
            ans: None,
            commit: "".to_string(),
            fg: Color::LightRed
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
        let ans = Editor::new("")
            .with_predefined_text(&self.commit)
            .with_render_config(
                RenderConfig::default()
                .with_prompt_prefix(Styled::new(self.msg.as_str()).with_fg(self.fg)),
            )
            .prompt()?;
        self.ans = Some(ans);
        Ok(())
    }
}
impl std::fmt::Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ans = self.ans.clone();
        let res = match &ans {
            Some(s) => s,
            None => ""
        };
        write!(f, "{res}")
    }
}

// pub fn inquire_commit_edit(messages: &str) -> ReviseResult<String> {
//     let msg = "You Really want to edit this commit manually?";
//     let ans = Editor::new("")
//         .with_predefined_text(messages)
//         .with_render_config(
//             RenderConfig::default()
//                 .with_prompt_prefix(Styled::new(msg).with_fg(Color::LightRed)),
//         )
//         .prompt()?;
//     Ok(ans)
// }
