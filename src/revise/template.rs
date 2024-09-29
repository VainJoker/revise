use std::fmt::Formatter;

use colored::Colorize;
use tera::{Context, Tera};
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
    pub async fn run(&mut self, cmd: &ReviseCommands) -> ReviseResult<String> {
        if cmd.ai.is_some() {
            self.run_action(cmd).await?;
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

    pub async fn run_action(
        &mut self,
        cmd: &ReviseCommands,
    ) -> ReviseResult<()> {
        let cfg = config::get_config();
        let Some(key) = cfg.api_key.get("gemini_key") else {
            return Err(anyhow::anyhow!("API key not found"));
        };

        let gemini = Gemini::new(key);

        let mut s = match cmd.ai.clone().unwrap() {
            AICommand::Translate(s) => s,
            AICommand::Generate => GitUtils::new().diff(&cmd.excludes)?,
        };
        // Fix: If the diff is empty, not directly ask user to input
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
    pub fn get_cicon(&self) -> String {
        config::get_config().get_emoji(&self.get_ctype()).unwrap()
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

    pub fn template(&self, color: bool) -> String {
        let cfg = config::get_config();
        let mut tera = Tera::default();
        tera.add_raw_template("template", &cfg.template).unwrap();
        let mut ctx = Context::new();
        if color {
            ctx.insert("commit_type", &self.get_ctype().green().to_string());
            ctx.insert("commit_icon", &self.get_cicon());
            match self.get_cscope() {
                Some(scope) => {
                    ctx.insert("commit_scope", &format!("{}", scope.yellow()));
                }
                None => {
                    ctx.insert("commit_scope", &Option::<String>::None);
                }
            }
            ctx.insert(
                "commit_subject",
                &self.get_csubject().bright_cyan().to_string(),
            );
            match self.get_cbody() {
                Some(body) => {
                    ctx.insert("commit_body", &body);
                }
                None => {
                    ctx.insert("commit_body", &Option::<String>::None);
                }
            }
            if let Some(breaking) = self.get_cbreaking() {
                ctx.insert(
                    "commit_breaking",
                    &format!(
                        "{}: {}",
                        "BREAKING CHANGE".bright_red(),
                        breaking.purple()
                    ),
                );
                ctx.insert(
                    "commit_breaking_symbol",
                    &"!".bright_red().to_string(),
                );
            } else {
                ctx.insert("commit_breaking", &Option::<String>::None);
                ctx.insert("commit_breaking_symbol", &Option::<String>::None);
            }
            match self.get_cissue() {
                Some(issue) => {
                    ctx.insert("commit_issue", &format!("{}", issue.blue()));
                }
                None => {
                    ctx.insert("commit_issue", &Option::<String>::None);
                }
            }
        } else {
            ctx.insert("commit_type", &self.get_ctype());
            ctx.insert("commit_icon", &self.get_cicon());
            ctx.insert("commit_scope", &self.get_cscope());
            ctx.insert("commit_subject", &self.get_csubject());
            ctx.insert("commit_body", &self.get_cbody());
            if let Some(breaking) = self.get_cbreaking() {
                ctx.insert(
                    "commit_breaking",
                    &format!("{}: {}", "BREAKING CHANGE", breaking),
                );
                ctx.insert("commit_breaking_symbol", &"!".to_string());
            } else {
                ctx.insert("commit_breaking", &Option::<String>::None);
                ctx.insert("commit_breaking_symbol", &Option::<String>::None);
            }
            ctx.insert("commit_issue", &self.get_cissue());
        }

        tera.render("template", &ctx).unwrap()
    }
}

impl std::fmt::Display for Template {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = self.template(false);
        write!(f, "{msg}")
    }
}

#[ignore]
#[test]
fn test_template() {
    config::initialize_config().unwrap_or_else(|e| {
        eprintln!("Load config err: {e}");
        std::process::exit(exitcode::CONFIG);
    });

    let t = Template {
        commit_type: commit_type::Part {
            ans: Some("feat".to_string()),
            ..Default::default()
        },
        commit_scope: commit_scope::Part {
            ans: Some("scope".to_string()),
            ..Default::default()
        },
        commit_subject: commit_subject::Part {
            ans: Some("add a new feature".to_string()),
            ..Default::default()
        },
        commit_body: commit_body::Part {
            ans: Some("add a new feature with a body".to_string()),
            ..Default::default()
        },
        commit_breaking: commit_breaking::Part {
            ans: Some("breaking change".to_string()),
            ..Default::default()
        },
        commit_issue: commit_issue::Part {
            ans: Some("#34".to_string()),
            ..Default::default()
        },
    };
    let s = t.template(true);
    println!("{s}");
    println!("{t}");
}
