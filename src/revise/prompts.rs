pub mod commit_body;
pub mod commit_breaking;
pub mod commit_confirm;
pub mod commit_edit;
pub mod commit_issue;
pub mod commit_scope;
pub mod commit_subject;
pub mod commit_type;
pub mod ai_generater;
pub mod ai_translator;

pub trait Inquire {
    fn inquire(&mut self) -> crate::error::ReviseResult<()>;
}
