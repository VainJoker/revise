use std::fmt::Formatter;

use anyhow::anyhow;
use colored::Colorize;
use inquire::{
    ui::{Color, RenderConfig, Styled},
    validator::{ErrorMessage, Validation},
    CustomType, Editor, Select, Text,
};

use crate::{config::ReviseConfig, error::ReviseResult};

#[derive(Debug, Default)]
pub struct ReviseCommit {
    pub commit_type: String,
    pub commit_scope: Option<String>,
    pub commit_subject: String,
    pub commit_body: Option<String>,
    pub commit_breaking: Option<String>,
    pub commit_issue: Option<String>,
    pub commit_confirm_commit: String,
    pub commit_status: CommitStatus,
    pub commit_message: String,
}

#[derive(Clone, Default, Debug)]
pub enum CommitStatus {
    #[default]
    Submit,
    Abort,
    Edit,
}

impl std::fmt::Display for CommitStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            CommitStatus::Submit => "Submit",
            CommitStatus::Abort => "Abort",
            CommitStatus::Edit => "Edit",
        };
        write!(f, "{}", s)
    }
}

impl std::str::FromStr for CommitStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "y" | "yes" | "" => Ok(CommitStatus::Submit),
            "n" | "no" => Ok(CommitStatus::Abort),
            "e" | "edit" => Ok(CommitStatus::Edit),
            &_ => Err(anyhow!("input error")),
        }
    }
}

impl std::fmt::Display for ReviseCommit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut msg = String::new();

        let _type = self.commit_type.to_string();
        msg.push_str(&_type);

        match (&self.commit_scope, &self.commit_breaking) {
            (None, None) => {
                msg.push_str(": ");
            }
            (None, Some(_)) => {
                let scope = format!("{}: ", "!");
                msg.push_str(&scope);
            }
            (Some(scope), None) => {
                let scope = format!("({}): ", scope);
                msg.push_str(&scope);
            }
            (Some(scope), Some(_)) => {
                let scope = format!("({}){}: ", scope, "!");
                msg.push_str(&scope);
            }
        }

        let subject = self.commit_subject.to_string();
        msg.push_str(&subject);

        match &self.commit_issue {
            Some(issues) => {
                let issues = format!("({})", issues);
                msg.push_str(&issues);
            }
            None => {}
        }
        match &self.commit_body {
            Some(body) => {
                let body = format!("\n{}", body);
                msg.push_str(&body);
            }
            None => {}
        }
        match &self.commit_breaking {
            Some(breaking) => {
                let breaking = format!("\n\n{}: {}", "BREAKING CHANGE", breaking);
                msg.push_str(&breaking);
            }
            None => {}
        }
        write!(f, "{}", msg)
    }
}

impl ReviseCommit {
    pub fn commit(&mut self, config: &ReviseConfig) -> ReviseResult<&Self> {
        self.inquire_commit_type(config)?;
        self.inquire_commit_scope(config)?;
        self.inquire_commit_subject()?;
        self.inquire_commit_body()?;
        self.inquire_commit_breaking()?;
        self.inquire_commit_issue()?;
        self.inquire_confirm_commit()?;
        Ok(self)
    }

    pub fn show(&self) -> String {
        let mut msg = String::new();

        let _type = format!("{}", &self.commit_type.green());
        msg.push_str(&_type);

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
                let scope = format!("({}){}: ", scope.yellow(), "!".bright_red());
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
                let body = format!("\n{}", body);
                msg.push_str(&body);
            }
            None => {}
        }
        match &self.commit_breaking {
            Some(breaking) => {
                let breaking = format!("\n\n{}: {}", "BREAKING CHANGE".red(), breaking);
                msg.push_str(&breaking);
            }
            None => {}
        }
        msg
    }

    pub fn inquire_commit_type(&mut self, config: &ReviseConfig) -> ReviseResult<()> {
        let msg = "Select the type of change that you're committing:";
        let type_options: Vec<String> = config.get_types();
        let ans = Select::new(msg, type_options.clone()).prompt()?;
        let idx = type_options
            .iter()
            .position(|s| *s == ans)
            .ok_or(anyhow!("Error Occurs when select committing type"))?;
        self.commit_type = config
            .get_type_key(idx)
            .ok_or(anyhow!("Error Occurs when select committing type"))?;
        Ok(())
    }

    pub fn inquire_commit_scope(&mut self, config: &ReviseConfig) -> ReviseResult<()> {
        let msg = "Denote the SCOPE of this change (optional):";
        let mut scope_options: Vec<String> = config.get_scopes();
        scope_options.push("empty".to_string());
        scope_options.push("custom".to_string());
        let ans = Select::new(msg, scope_options)
            .prompt()?;
            self.commit_scope = match ans.as_str() {
                "custom" => {
                    let msg = "Denote the SCOPE of this change:";
                    let ans = Text::new(msg).prompt()?;
                    Some(ans).filter(|a| !a.is_empty())
                }
                "empty" => None,
                _ => Some(ans),
            };
        Ok(())
    }

    pub fn inquire_commit_subject(&mut self) -> ReviseResult<()> {
        let msg = "Write a SHORT, IMPERATIVE tense description of the change:\n";
        let ans = Text::new(msg)
            .with_help_message("Infinity more chars allowed")
            .with_validator(|s: &str| {
                if s.is_empty() {
                    return Ok(Validation::Invalid(ErrorMessage::Custom(
                        "[ERROR] subject is required".to_string(),
                    )));
                }
                Ok(Validation::Valid)
            })
            .prompt()?;
        self.commit_subject = ans;
        Ok(())
    }

    pub fn inquire_commit_body(&mut self) -> ReviseResult<()> {
        let msg = "Provide a LONGER description of the change (optional):";
        let ans = Editor::new(msg)
            .with_formatter(&|submission| {
                let char_count = submission.chars().count();
                if char_count == 0 {
                    String::from("<skipped>")
                } else if char_count <= 20 {
                    submission.into()
                } else {
                    let mut substr: String = submission.chars().take(17).collect();
                    substr.push_str("...");
                    substr
                }
            })
            .with_render_config(RenderConfig::default().with_canceled_prompt_indicator(
                Styled::new("<skipped>").with_fg(Color::DarkYellow),
            ))
            .prompt()?;
            if ans.is_empty() {
                self.commit_body = None
            } else {
                self.commit_body = Some(ans)
            }
        Ok(())
    }

    pub fn inquire_commit_breaking(&mut self) -> ReviseResult<()> {
        let msg = "List any BREAKING CHANGES (optional):";
        let ans = Editor::new(msg)
            .with_formatter(&|submission| {
                let char_count = submission.chars().count();
                if char_count == 0 {
                    String::from("<skipped>")
                } else if char_count <= 20 {
                    submission.into()
                } else {
                    let mut substr: String = submission.chars().take(17).collect();
                    substr.push_str("...");
                    substr
                }
            })
        .prompt()?;
        if ans.is_empty() {
                self.commit_breaking = None
            } else {
                self.commit_breaking = Some(ans)
            }
        Ok(())
    }

    pub fn inquire_commit_issue(&mut self) -> ReviseResult<()> {
        let msg = "List any ISSUES by this change. E.g.= #31, #34: (optional)";
        let ans = Text::new(msg)
            .with_formatter(&|submission| {
                let char_count = submission.chars().count();
                if char_count == 0 {
                    String::from("<skipped>")
                } else if char_count <= 20 {
                    submission.into()
                } else {
                    let mut substr: String = submission.chars().take(17).collect();
                    substr.push_str("...");
                    substr
                }
            })
        .prompt()?;
        if ans.is_empty() {
            self.commit_issue = None
            } else {
                self.commit_issue = Some(ans)
            }
        Ok(())
    }

    pub fn inquire_confirm_commit(&mut self) -> ReviseResult<()> {
        self.commit_message = self.to_string();
        let cmsg = format!(
            "{}{}{}",
            "\n###--------------------------------------------------------###\n\n"
                .black()
                .bold()
                .italic(),
            self.show(),
            "\n\n###--------------------------------------------------------###\n"
                .black()
                .bold()
                .italic()
        );
        println!("{}", cmsg);
        let msg = "Are you sure you want to proceed with the commit above?";
        let ans = CustomType::<CommitStatus>::new(msg)
            .with_placeholder("y|n|e")
            .with_help_message("y for yes, n for no, e for edit")
            .with_error_message("Reply with 'y', 'n' or 'e'")
            .prompt()?;
        if let CommitStatus::Edit = ans {
            self.inquire_commit_edit()?
        } else {
            self.commit_status = ans;
        };
        Ok(())
    }

    pub fn inquire_commit_edit(&mut self) -> ReviseResult<()> {
        let msg = "You Really want to edit this commit by yourself?";
        let ans = Editor::new("")
            .with_predefined_text(&self.to_string())
            .with_render_config(
                RenderConfig::default()
                    .with_prompt_prefix(Styled::new(msg).with_fg(Color::LightRed)),
            )
            .prompt()?;
        self.commit_message = ans;
        Ok(())
    }
}
