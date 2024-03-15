use anyhow::anyhow;
use inquire::{Editor, Select, Text};

use crate::{config::ReviseConfig, error::ReviseResult};

#[derive(Debug, Default)]
pub struct ReviseCommit {
    pub commit_type: String,
    pub commit_scope: String,
    pub commit_custom_scope: Option<String>,
    pub commit_subject: String,
    pub commit_body: Option<String>,
    pub commit_breaking: Option<String>,
    pub commit_issue: Option<String>,
    pub commit_confirm_commit: String,
}

impl ReviseCommit {
    pub fn commit(&mut self, config: &ReviseConfig) -> ReviseResult<&Self> {
        self.inquire_commit_type(config)?;
        self.inquire_commit_scope(config)?;
        self.inquire_commit_custom_scope()?;
        self.inquire_commit_subject()?;
        self.inquire_commit_body()?;
        self.inquire_commit_breaking()?;
        self.inquire_commit_issue()?;
        self.inquire_confirm_commit()?;
        Ok(self)
    }

    pub fn inquire_commit_type(&mut self, config: &ReviseConfig) -> ReviseResult<()> {
        let msg = "Select the type of change that you're committing:";
        let type_options: Vec<String> = config.get_types();
        let ans = Select::new(msg, type_options.clone()).prompt()?;
        let idx = type_options
            .iter()
            .position(|s| *s == ans)
            .ok_or(anyhow!("Error Occurs when select committing type"))?;
        self.commit_type = config.get_type_key(idx).unwrap();
        Ok(())
    }

    pub fn inquire_commit_scope(&mut self, config: &ReviseConfig) -> ReviseResult<()> {
        let msg = "Denote the SCOPE of this change (optional):";
        let mut scope_options: Vec<String> = config.get_scopes();
        scope_options.push("empty".to_string());
        scope_options.push("custom".to_string());
        let ans = Select::new(msg, scope_options)
            .prompt()
            .map_err(|e| anyhow!("Error Occurs when select committing scope: {}", e))?;
        self.commit_scope = ans;
        Ok(())
    }

    pub fn inquire_commit_custom_scope(&mut self) -> ReviseResult<()> {
        if self.commit_scope != "custom" {
            self.commit_custom_scope = None;
            return Ok(());
        }
        let msg = "Denote the SCOPE of this change:";
        let ans = Text::new(msg)
            .prompt()
            .map_err(|e| anyhow!("Error Occurs when select committing custom scope: {}", e))?;
        self.commit_custom_scope = Some(ans);
        Ok(())
    }

    pub fn inquire_commit_subject(&mut self) -> ReviseResult<()> {
        let msg = "Write a SHORT, IMPERATIVE tense description of the change:";
        let ans = Text::new(msg)
            .prompt()
            .map_err(|e| anyhow!("Error Occurs when write committing custom subject: {}", e))?;
        self.commit_subject = ans;
        Ok(())
    }

    pub fn inquire_commit_body(&mut self) -> ReviseResult<()> {
        let msg = "Provide a LONGER description of the change (optional):";
        let description = Editor::new(msg)
            .prompt()
            .map_err(|e| anyhow!("Error Occurs when write committing custom body: {}", e))?;
        if description.is_empty() {
            self.commit_body = None
        } else {
            self.commit_body = Some(description)
        }
        Ok(())
    }

    pub fn inquire_commit_breaking(&mut self) -> ReviseResult<()> {
        let msg = "List any BREAKING CHANGES (optional):";
        let ans = Text::new(msg)
            .prompt()
            .map_err(|e| anyhow!("Error Occurs when describe breaking changes: {}", e))?;
        if ans.is_empty() {
            self.commit_breaking = None
        } else {
            self.commit_breaking = Some(ans)
        }
        Ok(())
    }

    pub fn inquire_commit_issue(&mut self) -> ReviseResult<()> {
        let msg = "List any ISSUES by this change. E.g.= #31, #34:\n";
        let ans = Text::new(msg)
            .prompt()
            .map_err(|e| anyhow!("Error Occurs when select related issues: {}", e))?;
        if ans.is_empty() {
            self.commit_issue = None
        } else {
            self.commit_issue = Some(ans)
        }
        Ok(())
    }

    pub fn inquire_confirm_commit(&mut self) -> ReviseResult<()> {
        let msg = "Are you sure you want to proceed with the commit above?";
        let _ans = Text::new(msg)
            .prompt()
            .map_err(|e| anyhow!("Error Occurs when confirm commit: {}", e))?;
        Ok(())
    }
}
