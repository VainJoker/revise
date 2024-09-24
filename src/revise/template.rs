use std::fmt::Formatter;

use colored::Colorize;
use tokio::task;

use super::prompts::{
    commit_ai, commit_body, commit_breaking, commit_confirm, commit_issue,
    commit_scope, commit_subject, commit_translate, commit_type,
};
use crate::{
    ai::{gemini::Gemini, AI},
    config,
    error::ReviseResult,
    git::GitUtils,
    revise::prompts::Inquire,
    AICommand, ReviseCommands,
};

#[derive(Debug, Default, Clone)]
pub struct Template {
    pub commit_type: commit_type::Part,
    pub commit_scope: commit_scope::Part,
    pub commit_subject: commit_subject::Part,
    pub commit_body: commit_body::Part,
    pub commit_breaking: commit_breaking::Part,
    pub commit_issue: commit_issue::Part,
}

impl Template {
    pub async fn run(
        &mut self,
        cmd: &Option<ReviseCommands>,
    ) -> ReviseResult<String> {
        if let Some(a) = cmd {
            self.run_action(&a.ai).await?;
        } else {
            self.run_default()?;
        }
        let mut confirm = commit_confirm::Part::new(self.clone());
        confirm.inquire()?;
        Ok(confirm.ans.unwrap())
    }

    pub fn run_default(&mut self) -> ReviseResult<()> {
        self.commit_type.inquire()?;
        self.commit_scope.inquire()?;
        self.commit_subject.inquire()?;
        self.commit_body.inquire()?;
        self.commit_breaking.inquire()?;
        self.commit_issue.inquire()?;
        Ok(())
    }

    pub async fn run_action(&mut self, cmd: &AICommand) -> ReviseResult<()> {
        let cfg = config::get_config();
        let Some(key) = cfg.api_key.get("gemini_key") else {
            return Err(anyhow::anyhow!("API key not found"));
        };

        let gemini = Gemini::new(key);
        let mut s = match cmd {
            AICommand::Translate(s) => s.to_string(),
            AICommand::Generate => GitUtils::new().diff()?,
        };
        if s.is_empty() {
            let mut translate = commit_translate::Part::new();
            translate.inquire()?;
            s = match translate.ans {
                Some(s) => s,
                None => {
                    return Err(anyhow::anyhow!("Translate message is empty"));
                }
            };
        }
        let handle =
            task::spawn(async move { gemini.generate_response(&s).await });

        self.commit_type.inquire()?;
        self.commit_scope.inquire()?;
        self.commit_breaking.inquire()?;
        self.commit_issue.inquire()?;

        let res = handle.await??;

        let mut ai = commit_ai::Part::new(res.keys().cloned().collect());
        ai.inquire()?;
        let ai_ans = res.get(&ai.ans.clone().unwrap()).unwrap();
        self.commit_subject.ans = Some(ai_ans.message.clone());
        self.commit_body.ans = Some(ai_ans.body.clone());
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

        if let Some(issues) = &self.get_cissue() {
            let issues = format!("({})", issues.blue());
            msg.push_str(&issues);
        }
        if let Some(body) = &self.get_cbody() {
            let body = format!("\n\n{body}");
            msg.push_str(&body);
        }
        if let Some(breaking) = &self.get_cbreaking() {
            let breaking =
                format!("\n\n{}: {}", "BREAKING CHANGE".red(), breaking);
            msg.push_str(&breaking);
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

        if let Some(issues) = &self.get_cissue() {
            let issues = format!("({issues})");
            msg.push_str(&issues);
        }
        if let Some(body) = &self.get_cbody() {
            let body = format!("\n\n{body}");
            msg.push_str(&body);
        }
        if let Some(breaking) = &self.get_cbreaking() {
            let breaking = format!("\n\n{}: {}", "BREAKING CHANGE", breaking);
            msg.push_str(&breaking);
        }
        write!(f, "{msg}")
    }
}
