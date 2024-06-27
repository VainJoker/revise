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
    pub fg: Color,
}

impl Part {
    pub fn new() -> Self {
        Self {
            msg: "Provide a LONGER description of the change (optional):".to_string(),
            ans: None,
            fg: Color::DarkYellow
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
            "<skipped>" | "" => {},
            _ => {self.ans = Some(ans);}
        }

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

//     fn inquire(&mut self, _: &ReviseConfig) -> ReviseResult<Option<String>> {
//         }
// }

// pub fn inquire_commit_body() -> ReviseResult<Option<String>> {
//     let msg = "Provide a LONGER description of the change (optional):";
//     let ans = Editor::new(msg)
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
//         .with_render_config(
//             RenderConfig::default().with_canceled_prompt_indicator(
//                 Styled::new("<skipped>").with_fg(Color::DarkYellow),
//             ),
//         )
//         .prompt()?;
//
//     match &*ans {
//         "<skipped>" | "" => Ok(None),
//         _ => Ok(Some(ans)),
//     }
// }
#[cfg(test)]

mod tests{

    #[test]
    fn test_format_with_option() {
        struct A {
            a: Option<String>
        }

        impl std::fmt::Display for A {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let s = match &self.a{
                    Some(l) => l,
                    None => ""
                };
                write!(f,"{s}")
            }
        }
        let a = A{a: Some("123456".to_owned())};
        let b = A{a: None};
        println!("{a}{b}{a}");
        // println!("{b}");

    }

}
