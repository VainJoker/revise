use inquire::Text;

use super::Inquire;
use crate::error::ReviseResult;

#[derive(Debug, Clone)]
pub struct Part {
    pub msg: String,
    pub ans: Option<String>,
}

impl Part {
    pub fn new() -> Self {
        Self {
            msg: "List any ISSUES by this change. E.g.= #31, #34:".to_string(),
            ans: None,
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
        let ans = Text::new(&self.msg)
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
            "<skipped>" | "" => {}
            _ => self.ans = Some(ans),
        };
        Ok(())
    }
}
