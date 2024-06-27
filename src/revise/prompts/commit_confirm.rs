use crate::{
    error::ReviseResult,
    revise::{
        // prompts::commit_edit::inquire_commit_edit, status::CommitStatus,
        // template::CommitTemplate,
    },
};

use super::Inquire;

#[derive(Debug,Clone)]
pub struct Part{
    pub msg: String,
    pub ans: Option<String>,
}

impl Part {
    pub fn new() -> Self {
        Self {
            msg: "Provide a LONGER description of the change (optional):".to_string(),
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
        todo!()
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



// pub fn inquire_confirm_commit(
//     template: &CommitTemplate,
// ) -> ReviseResult<String> {
//     let res_msg = format!(
//         "{}{}{}",
//         "\n###--------------------------------------------------------###\n\n"
//             .black()
//             .bold()
//             .italic(),
//         template.show(),
//         "\n\n###--------------------------------------------------------###\n"
//             .black()
//             .bold()
//             .italic()
//     );
//     println!("{res_msg}");
//     let msg = "Are you sure you want to proceed with the commit above?";
//     let ans = CustomType::<CommitStatus>::new(msg)
//         .with_placeholder("y|n|e")
//         .with_help_message("y for yes, n for no, e for edit")
//         .with_error_message("Reply with 'y', 'n' or 'e'")
//         .prompt()?;
//
//     match ans {
//         CommitStatus::Edit => inquire_commit_edit(&template.to_string()),
//         _ => Ok(template.to_string()),
//     }
// }
