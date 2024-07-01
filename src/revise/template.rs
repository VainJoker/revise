use std::fmt::Formatter;

use colored::Colorize;

use super::prompts::{
    commit_body, commit_breaking, commit_issue, commit_scope, commit_subject,
    commit_type,
};
use crate::{error::ReviseResult, revise::prompts::Inquire};

// #[derive(Debug)]
// pub struct Template {
//     pub commit_type: Ctype,
//     pub commit_scope: Option<String>,
//     pub commit_subject: CSubject,
//     pub commit_body: Option<String>,
//     pub commit_breaking: Option<String>,
//     pub commit_issue: Option<String>,
// }
#[derive(Debug, Default, Clone)]
pub struct Template {
    pub commit_type: commit_type::Part,
    pub commit_scope: commit_scope::Part,
    pub commit_subject: commit_subject::Part,
    pub commit_body: commit_body::Part,
    pub commit_breaking: commit_breaking::Part,
    pub commit_issue: commit_issue::Part,
}

// #[derive(Default)]
// pub struct Template {
//     fields: Vec<Box<dyn Inquire>>,
// }

impl Template {
    pub fn run(&mut self) -> ReviseResult<()> {
        self.commit_type.inquire()?;
        self.commit_scope.inquire()?;
        self.commit_subject.inquire()?;
        self.commit_body.inquire()?;
        self.commit_breaking.inquire()?;
        self.commit_issue.inquire()?;
        Ok(())
    }

    pub fn get_ctype(&self) -> String {
        self.commit_type.ans.clone().unwrap()
    }
    pub fn get_cscope(&self) -> Option<String> {
        self.commit_scope.ans.clone()
    }
    pub fn get_csubject(&self) -> String {
        self.commit_subject.ans.clone().unwrap()
    }
    pub fn get_cbody(&self) -> Option<String> {
        self.commit_body.ans.clone()
    }
    pub fn get_cbreaking(&self) -> Option<String> {
        self.commit_breaking.ans.clone()
    }
    pub fn get_cissue(&self) -> Option<String> {
        self.commit_issue.ans.clone()
    }

    pub fn show(&self) -> String {
        let mut msg = String::new();

        String::push_str(&mut msg, &format!("{}", &self.get_ctype().green()));

        match (&self.get_cscope(), &self.get_cbreaking()) {
            (None, None) => {
                msg.push_str(": ");
            }
            (None, Some(_)) => {
                let scope = format!("{}: ", "!".bright_red());
                msg.push_str(&scope);
            }
            (Some(scope), None) => {
                let scope = format!("({}): ", scope.yellow());
                msg.push_str(&scope);
            }
            (Some(scope), Some(_)) => {
                let scope =
                    format!("({}){}: ", scope.yellow(), "!".bright_red());
                msg.push_str(&scope);
            }
        }

        let subject = format!("{}", &self.get_csubject().bright_cyan());
        msg.push_str(&subject);

        match &self.get_cissue() {
            Some(issues) => {
                let issues = format!("({})", issues.blue());
                msg.push_str(&issues);
            }
            None => {}
        }
        match &self.get_cbody() {
            Some(body) => {
                let body = format!("\n{body}");
                msg.push_str(&body);
            }
            None => {}
        }
        match &self.get_cbreaking() {
            Some(breaking) => {
                let breaking =
                    format!("\n\n{}: {}", "BREAKING CHANGE".red(), breaking);
                msg.push_str(&breaking);
            }
            None => {}
        }
        msg
    }
}

impl std::fmt::Display for Template {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::new();

        // let ctype = &self.commit_type;
        msg.push_str(&self.get_ctype());

        match (&self.get_cscope(), &self.get_cbreaking()) {
            (None, None) => {
                msg.push_str(": ");
            }
            (None, Some(_)) => {
                let scope = format!("{}: ", "!");
                msg.push_str(&scope);
            }
            (Some(scope), None) => {
                let scope = format!("({scope}): ");
                msg.push_str(&scope);
            }
            (Some(scope), Some(_)) => {
                let scope = format!("({}){}: ", scope, "!");
                msg.push_str(&scope);
            }
        }

        // let subject = &self.commit_subject;
        msg.push_str(&self.get_csubject());

        match &self.get_cissue() {
            Some(issues) => {
                let issues = format!("({issues})");
                msg.push_str(&issues);
            }
            None => {}
        }
        match &self.get_cbody() {
            Some(body) => {
                let body = format!("\n{body}");
                msg.push_str(&body);
            }
            None => {}
        }
        match &self.get_cbreaking() {
            Some(breaking) => {
                let breaking =
                    format!("\n\n{}: {}", "BREAKING CHANGE", breaking);
                msg.push_str(&breaking);
            }
            None => {}
        }
        write!(f, "{msg}")
    }
}
