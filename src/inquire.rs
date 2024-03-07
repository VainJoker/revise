use inquire::{Editor, Select, Text};

use crate::config::Config;

pub struct Commit {
    pub commit_type: String,
    pub commit_scope: Option<String>,
    pub commit_custom_scope: Option<String>,
    pub commit_subject: String,
    pub commit_body: Option<String>,
    pub commit_breaking: Option<String>,
    pub commit_footer_prefixes_select: Option<String>,
    pub commit_footer: Option<String>,
    pub commit_confirm_commit: String,
}

impl Default for Commit {
    fn default() -> Self {
        Self::new()
    }
}

impl Commit {
    pub fn new() -> Self {
        Commit {
            commit_type: "".to_string(),
            commit_scope: None,
            commit_custom_scope: None,
            commit_subject: "".to_string(),
            commit_body: None,
            commit_breaking: None,
            commit_footer_prefixes_select: None,
            commit_footer: None,
            commit_confirm_commit: "".to_string(),
        }
    }

    pub fn inquire_commit_type(&mut self, config: Config) {
        let msg = "Select the type of change that you're committing:";
        let type_options: Vec<String> = config.get_types();
        let ans = Select::new(msg, type_options)
            .prompt()
            .expect("Error Occurs when select commit type");
        self.commit_type = ans
            .split(':')
            .next()
            .expect("types config error")
            .to_string();
    }

    pub fn inquire_commit_scope(&mut self, config: Config) {
        let msg = "Denote the SCOPE of this change (optional):";
        let mut scope_options: Vec<String> = config.scopes;
        if scope_options.is_empty() {
            scope_options.push("empty".to_string());
            scope_options.push("custom".to_string());
        }
        let ans = Select::new(msg, scope_options)
            .prompt()
            .expect("Error Occurs when select commit scope");
        self.commit_scope = Some(ans);
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
}
