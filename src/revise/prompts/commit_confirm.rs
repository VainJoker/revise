use colored::Colorize;
use inquire::CustomType;

use super::Inquire;
use crate::{
    error::ReviseResult,
    revise::{prompts::commit_edit, status::Status, template::Template},
};

#[derive(Debug, Clone)]
pub struct Part {
    pub msg: String,
    pub ans: Option<String>,
    pub template: Template,
}

impl Part {
    pub fn new(template: Template) -> Self {
        Self {
            msg: "Provide a LONGER description of the change (optional):"
                .to_string(),
            ans: None,
            template,
        }
    }
}

// impl Default for Part {
//     fn default() -> Self {
//         Self::new()
//     }
// }

impl Inquire for Part {
    fn inquire(&mut self) -> ReviseResult<()> {
        let res_msg = format!(
            "{}{}{}",
            "\n###--------------------------------------------------------###\n\n"
            .black()
            .bold()
            .italic(),
            self.template.show(),
            "\n\n###--------------------------------------------------------###\n"
            .black()
            .bold()
            .italic()
        );
        println!("{res_msg}");
        let msg = "Are you sure you want to proceed with the commit above?";
        let ans = CustomType::<Status>::new(msg)
            .with_placeholder("y|n|e")
            .with_help_message("y for yes, n for no, e for edit")
            .with_error_message("Reply with 'y', 'n' or 'e'")
            .prompt()?;

        match ans {
            Status::Edit => {
                let mut cedit =
                    commit_edit::Part::new(self.template.to_string());
                cedit.inquire().unwrap();
                self.ans = cedit.ans;
            }
            // inquire_commit_edit(&self.template.to_string()),
            _ => self.ans = Some(self.template.to_string()),
        };
        Ok(())
    }
}
