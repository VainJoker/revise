use inquire::Text;

use crate::error::ReviseResult;

use super::Inquire;


pub struct Issue{
    pub msg: String,
    pub ans: Option<String>,
}

impl Issue{
    pub fn new() -> Self {
        Self {
            msg: "List any ISSUES by this change. E.g.= #31, #34:".to_string(),
            ans: None,
        }
    }
}

impl Default for Issue {
    fn default() -> Self {
        Self::new()
    }
}

impl Inquire for Issue {
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
            "<skipped>" | "" => {},
            _ => self.ans = Some(ans),
        };
        Ok(())
    }
}

// pub fn inquire_commit_issue() -> ReviseResult<Option<String>> {
//     let msg = "List any ISSUES by this change. E.g.= #31, #34: (optional)";
//     let ans = Text::new(msg)
//         .with_formatter(&|submission| {
//             let char_count = submission.chars().count();
//             if char_count == 0 {
//                 "<skipped>".to_string()
//             } else if char_count <= 20 {
//                 submission.into()
//             } else {
//                 format!("{}...", &submission[..17])
//             }
//         })
//     .prompt()?;
//     match &*ans {
//         "<skipped>" | "" => Ok(None),
//         _ => Ok(Some(ans)),
//     }
// }
