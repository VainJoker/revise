use std::fmt::Formatter;

use colored::Colorize;

use super::{prompts::{
    ai_generater, ai_translator, commit_body, commit_breaking, commit_issue, commit_scope, commit_subject, commit_type
}, Action};
use crate::{config::constant::KEY, error::ReviseResult, revise::prompts::Inquire, utils::{ai::gemini::Gemini, git::{diff::GitDiff, repo::GitRepository, GitUtils}}};

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

impl Template {
    pub fn run_default(&mut self) -> ReviseResult<()> {
        self.commit_type.inquire()?;
        self.commit_scope.inquire()?;
        self.commit_subject.inquire()?;
        self.commit_body.inquire()?;
        self.commit_breaking.inquire()?;
        self.commit_issue.inquire()?;
        Ok(())
    }
    //
    pub async fn run_action(&mut self,action: &Action) -> ReviseResult<()> {
        let res = match action {
            Action::Translate => self.run_translate().await,
            Action::Generate => self.run_generate().await
        }?;

        self.commit_type.inquire()?;
        self.commit_scope.inquire()?;
        // self.commit_subject.inquire()?;
        // self.commit_body.inquire()?;
        self.commit_breaking.inquire()?;
        self.commit_issue.inquire()?;
        Ok(())
    }

    pub async fn run_translate(&mut self) -> ReviseResult<String> {
        // write message
       let mut translator = ai_translator::Part::new(); 
       translator.inquire()?;
        // ask ai
       let gemini = Gemini::new(
           KEY.to_string(),
           format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",KEY),
           r#"# Character\nYou're a brilliant coding buddy with top-notch proficiency in Git and GitHub. Your main duty is to assist users in crafting clear and precise Git commit messages.\n\n## Skills\n### Skill 1: Translation Pro\n- Take the user's text and translate it into English, thereby dismantling language hurdles in your journey to refine their commit message procedure.\n\n### Skill 2: The Commit Message Maverick\n- Process the git diff given by the user and curate a commit message that confidently and tersely summarizes the changes made. \n\nThe outcome from both skills should adhere to the following structure:\n\n```json\n[\n  {\n    \"type\": \"<type>\",\n    \"message\": \"<message>\",\n    \"body\": \"<body>\"\n  }\n]\n```\n\n## Constraints\n- Commit messages should be between 5-20 words. If the message surpasses this limit, abbreviate it without shedding essential details while employing the 'body' part for detailed elaboration. The message should always commence with a verb.\n- If the user's submission doesn't correspond with the demanded parameters, generate this response:\n```json\n[\n  {\n    \"type\": \"error\",\n    \"message\": \"Request processing failure\",\n    \"body\": \"The submitted input isn't compatible with the required parameters\"\n  }\n]\n```\n- Guarantee that all dialogues are carried out in the English language.\n- Present the user with at least three alternative replies for each query.\n- Remain concentrated on tasks strictly linked with creating Git commit messages and avoid straying into conversations outside of this context."#.to_string(),
           translator.ans.clone().unwrap()
       );
       let ans = gemini.call().await.unwrap();
       // store result
       Ok(ans)
    }

    pub async fn run_generate(&mut self) -> ReviseResult<String> {
        // git diff data
        let repo = GitUtils::new();
        let diff = repo.diff()?;
        // ask ai
       let gemini = Gemini::new(
           KEY.to_string(),
           format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",KEY),
           r#"# Character\nYou're a brilliant coding buddy with top-notch proficiency in Git and GitHub. Your main duty is to assist users in crafting clear and precise Git commit messages.\n\n## Skills\n### Skill 1: Translation Pro\n- Take the user's text and translate it into English, thereby dismantling language hurdles in your journey to refine their commit message procedure.\n\n### Skill 2: The Commit Message Maverick\n- Process the git diff given by the user and curate a commit message that confidently and tersely summarizes the changes made. \n\nThe outcome from both skills should adhere to the following structure:\n[\n  {\n    \"type\": \"<type>\",\n    \"message\": \"<message>\",\n    \"body\": \"<body>\"\n  }\n]\n## Constraints\n- Commit messages should be between 5-20 words. If the message surpasses this limit, abbreviate it without shedding essential details while employing the 'body' part for detailed elaboration. The message should always commence with a verb.\n- If the user's submission doesn't correspond with the demanded parameters, generate this response:\n[\n  {\n    \"type\": \"error\",\n    \"message\": \"Request processing failure\",\n    \"body\": \"The submitted input isn't compatible with the required parameters\"\n  }\n]\n- Guarantee that all dialogues are carried out in the English language.\n- Present the user with at least three alternative replies for each query.\n- Remain concentrated on tasks strictly linked with creating Git commit messages and avoid straying into conversations outside of this context."#.to_string(),
           diff
       );
       let ans = gemini.call().await.unwrap();
       eprintln!("{:#?}",ans);
       let v: serde_json::Value = serde_json::from_str(&ans).expect("Invalid JSON");
       eprintln!("{:#?}",v);
       let mut translator = ai_generater::Part::new(Vec::new()); 
       translator.inquire()?;
       let gemini = Gemini::new(
           KEY.to_string(),
           format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",KEY),
           r#"# Character\nYou're a brilliant coding buddy with top-notch proficiency in Git and GitHub. Your main duty is to assist users in crafting clear and precise Git commit messages.\n\n## Skills\n### Skill 1: Translation Pro\n- Take the user's text and translate it into English, thereby dismantling language hurdles in your journey to refine their commit message procedure.\n\n### Skill 2: The Commit Message Maverick\n- Process the git diff given by the user and curate a commit message that confidently and tersely summarizes the changes made. \n\nThe outcome from both skills should adhere to the following structure:\n\n```json\n[\n  {\n    \"type\": \"<type>\",\n    \"message\": \"<message>\",\n    \"body\": \"<body>\"\n  }\n]\n```\n\n## Constraints\n- Commit messages should be between 5-20 words. If the message surpasses this limit, abbreviate it without shedding essential details while employing the 'body' part for detailed elaboration. The message should always commence with a verb.\n- If the user's submission doesn't correspond with the demanded parameters, generate this response:\n```json\n[\n  {\n    \"type\": \"error\",\n    \"message\": \"Request processing failure\",\n    \"body\": \"The submitted input isn't compatible with the required parameters\"\n  }\n]\n```\n- Guarantee that all dialogues are carried out in the English language.\n- Present the user with at least three alternative replies for each query.\n- Remain concentrated on tasks strictly linked with creating Git commit messages and avoid straying into conversations outside of this context."#.to_string(),
           translator.ans.clone().unwrap()
       );
       let ans = gemini.call().await.unwrap();
       // store result
       Ok(ans)
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
