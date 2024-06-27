pub mod commit_body;
pub mod commit_breaking;
pub mod commit_confirm;
pub mod commit_edit;
pub mod commit_issue;
pub mod commit_scope;
pub mod commit_subject;
pub mod commit_type;

use crate::error::ReviseResult;

pub trait Inquire {
    fn inquire(&mut self) -> ReviseResult<()>;
}



// #[derive(Debug, Default, Clone)]
// pub struct Template {
//     pub ctype: CType,
//     pub cscope: CScope,
//     pub csubject: CSubject,
//     pub cbody: CBody,
//     pub cbreaking: CBreaking,
//     pub cissue: CIssue
// }
//
// impl std::fmt::Display for Template {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let mut msg = String::new();
//
//         let ctype = &self.ctype.ans;
//         msg.push_str(ctype);
//         let scope = self.cscope.ans;
//         let breaking = self.cbreaking.ans;
//
//         match (&scope, &breaking) {
//             (None, None) => {
//                 msg.push_str(": ");
//             }
//             (None, Some(_)) => {
//                 msg.push_str(&format!("{}: ", "!"));
//             }
//             (Some(s), None) => {
//                 msg.push_str(&format!("({s}): "));
//             }
//             (Some(s), Some(_)) => {
//                 msg.push_str(&format!("({}){}: ", s, "!"));
//             }
//         }
//
//         let subject = &self.commit_subject;
//         msg.push_str(subject);
//
//         match &self.commit_issue {
//             Some(issues) => {
//                 let issues = format!("({issues})");
//                 msg.push_str(&issues);
//             }
//             None => {}
//         }
//         match &self.commit_body {
//             Some(body) => {
//                 let body = format!("\n{body}");
//                 msg.push_str(&body);
//             }
//             None => {}
//         }
//         match &self.commit_breaking {
//             Some(breaking) => {
//                 let breaking =
//                     format!("\n\n{}: {}", "BREAKING CHANGE", breaking);
//                 msg.push_str(&breaking);
//             }
//             None => {}
//         }
//         write!(f, "{msg}")
//     }
// }
