use std::fmt::Formatter;

use colored::Colorize;

use crate::{
    config::get_config,
    error::ReviseResult,
    revise::prompts::{
        commit_body::inquire_commit_body,
        commit_breaking::inquire_commit_breaking,
        commit_issue::inquire_commit_issue, commit_scope::inquire_commit_scope,
        commit_subject::inquire_commit_subject,
        commit_type::inquire_commit_type,
    },
};

#[derive(Debug, Default, Clone)]
pub struct CommitTemplate {
    pub commit_type: String,
    pub commit_scope: Option<String>,
    pub commit_subject: String,
    pub commit_body: Option<String>,
    pub commit_breaking: Option<String>,
    pub commit_issue: Option<String>,
}

impl CommitTemplate {
    pub fn run(&mut self) -> ReviseResult<()> {
        let config = get_config();
        self.commit_type = inquire_commit_type(config)?;
        self.commit_scope = inquire_commit_scope(config)?;
        self.commit_subject = inquire_commit_subject()?;
        self.commit_body = inquire_commit_body()?;
        self.commit_breaking = inquire_commit_breaking()?;
        self.commit_issue = inquire_commit_issue()?;
        Ok(())
    }
    pub fn show(&self) -> String {
        let mut msg = String::new();

        let ctype = format!("{}", &self.commit_type.green());
        msg.push_str(&ctype);

        match (&self.commit_scope, &self.commit_breaking) {
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

        let subject = format!("{}", &self.commit_subject.bright_cyan());
        msg.push_str(&subject);

        match &self.commit_issue {
            Some(issues) => {
                let issues = format!("({})", issues.blue());
                msg.push_str(&issues);
            }
            None => {}
        }
        match &self.commit_body {
            Some(body) => {
                let body = format!("\n{body}");
                msg.push_str(&body);
            }
            None => {}
        }
        match &self.commit_breaking {
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

impl std::fmt::Display for CommitTemplate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::new();

        let ctype = &self.commit_type;
        msg.push_str(ctype);

        match (&self.commit_scope, &self.commit_breaking) {
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

        let subject = &self.commit_subject;
        msg.push_str(subject);

        match &self.commit_issue {
            Some(issues) => {
                let issues = format!("({issues})");
                msg.push_str(&issues);
            }
            None => {}
        }
        match &self.commit_body {
            Some(body) => {
                let body = format!("\n{body}");
                msg.push_str(&body);
            }
            None => {}
        }
        match &self.commit_breaking {
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
