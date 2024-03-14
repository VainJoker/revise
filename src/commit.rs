use inquire::{Editor, Select, Text};

use crate::{config::ReviseConfig, error::ReviseResult};

#[derive(Debug)]
pub struct ReviseCommit {
    pub commit_type: String,
    pub commit_scope: Option<String>,
    pub commit_custom_scope: Option<String>,
    pub commit_subject: String,
    pub commit_body: Option<String>,
    pub commit_breaking: Option<String>,
    pub commit_footer_prefixes_select: Option<String>,
    pub commit_custom_footer_prefix: Option<String>,
    pub commit_footer: Option<String>,
    pub commit_confirm_commit: String,
}

impl Default for ReviseCommit {
    fn default() -> Self {
        Self {
            commit_type: "".to_string(),
            commit_scope: None,
            commit_custom_scope: None,
            commit_subject: "".to_string(),
            commit_body: None,
            commit_breaking: None,
            commit_footer_prefixes_select: None,
            commit_custom_footer_prefix: None,
            commit_footer: None,
            commit_confirm_commit: "".to_string(),
        }
    }
}

impl ReviseCommit {
    pub fn commit(&mut self, config: &ReviseConfig) -> ReviseResult<&Self> {
        self.inquire_commit_type(config)?;
        self.inquire_commit_scope(config)?;
        Ok(self)
    }

    pub fn inquire_commit_type(&mut self, config: &ReviseConfig) -> ReviseResult<()> {
        let msg = "Select the type of change that you're committing:";
        let type_options: Vec<String> = config.get_types();
        let ans = Select::new(msg, type_options.clone()).prompt()?;
        let idx = type_options
            .iter()
            .position(|s| *s == ans)
            .ok_or(anyhow::anyhow!("Select committing type"))?;
        self.commit_type = config.get_type_key(idx).unwrap();
        Ok(())
    }

    pub fn inquire_commit_scope(&mut self, config: &ReviseConfig) -> ReviseResult<()> {
        let msg = "Denote the SCOPE of this change (optional):";
        let mut scope_options: Vec<String> = config.get_scopes();
        if scope_options.is_empty() {
            scope_options.push("empty".to_string());
            scope_options.push("custom".to_string());
        }
        let ans = Select::new(msg, scope_options).prompt()?;
        self.commit_scope = Some(ans);
        Ok(())
    }

    pub fn inquire_commit_custom_scope(&mut self) {
        let msg = "Denote the SCOPE of this change:";
        let ans = Text::new(msg)
            .prompt()
            .expect("Error Occurs when select commit custom scope");
        self.commit_custom_scope = Some(ans);
    }

    pub fn inquire_commit_subject(&mut self) {
        let msg = "Write a SHORT, IMPERATIVE tense description of the change:";
        let ans = Text::new(msg)
            .prompt()
            .expect("Error Occurs when select commit subject");
        self.commit_subject = ans;
    }

    pub fn inquire_commit_body(&mut self) {
        let _msg = "Provide a LONGER description of the change (optional):";
        let _description = Editor::new("Description:")
            .prompt()
            .expect("Error Occurs when select commit subject");
    }
    pub fn inquire_commit_breaking(&mut self) {
        let _msg = "List any BREAKING CHANGES (optional):";
        let _description = Editor::new("Description:")
            .prompt()
            .expect("Error Occurs when select commit subject");
    }
    pub fn inquire_commit_footer_prefixes_select(&mut self) {
        let _msg = "Select the ISSUES type of changeList by this change (optional):";
        let _description = Editor::new("Description:")
            .prompt()
            .expect("Error Occurs when select commit subject");
    }

    pub fn inquire_commit_footer_custom_footer_prefix(&mut self) {
        let _msg = " Input ISSUES prefix:";
        let _description = Editor::new("Description:")
            .prompt()
            .expect("Error Occurs when select commit subject");
    }

    pub fn inquire_commit_footer(&mut self) {
        let _msg = "List any ISSUES by this change. E.g.= #31, #34:\n";
        let _description = Editor::new("Description:")
            .prompt()
            .expect("Error Occurs when select commit subject");
    }

    pub fn inquire_confirm_commit(&mut self) {
        let _msg = "Are you sure you want to proceed with the commit above?";
        let _description = Editor::new("Description:")
            .prompt()
            .expect("Error Occurs when select commit subject");
    }
}

#[test]
fn test_search_type() {
    let config = ReviseConfig::load_config().unwrap();
    let type_options: Vec<String> = config.get_types();
    let idx = type_options
        .iter()
        .position(|s| s == "feat:     A new feature")
        .unwrap();
    assert_eq!(0, idx)
}
