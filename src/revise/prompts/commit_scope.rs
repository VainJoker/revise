use inquire::{Select, Text};

use crate::{config::ReviseConfig, error::ReviseResult};

use super::Inquire;


pub struct Scope{
    pub msg: String,
    pub ans: Option<String>,
    pub options: Vec<String>
}

impl Scope {
    pub fn new() -> Self {
        Self {
            msg: "Denote the SCOPE of this change (optional):".to_string(),
            ans: None,
            options: Vec::new()
        }
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

impl Inquire for Scope {
    fn inquire(&mut self) -> ReviseResult<()> {

        // // Prepend "empty" if not present
        // if !scope_options.contains(&"empty".to_string()) {
        //     scope_options.insert(0, "empty".to_string());
        // }
        //
        // // Append "custom" if not present
        // if !scope_options.contains(&"custom".to_string()) {
        //     scope_options.push("custom".to_string());
        // }

        let ans = Select::new(&self.msg, self.options.clone()).prompt()?;

        // TODO:
        // if ans == "custom" {
        //     let ans = Text::new("Denote the SCOPE of this change:").prompt()?;
        //     Ok(Some(ans).filter(|a| !a.is_empty()))
        // } else if ans == "empty" {
        //     Ok(None)
        // } else {
        //     Ok(Some(ans))
        // };
        self.ans = Some(ans);
        Ok(())
    }
}

// pub fn inquire_commit_scope(
//     config: &ReviseConfig,
// ) -> ReviseResult<Option<String>> {
//     let msg = "Denote the SCOPE of this change (optional):";
//     let mut scope_options: Vec<String> = config.get_scopes();
//
//     // Prepend "empty" if not present
//     if !scope_options.contains(&"empty".to_string()) {
//         scope_options.insert(0, "empty".to_string());
//     }
//
//     // Append "custom" if not present
//     if !scope_options.contains(&"custom".to_string()) {
//         scope_options.push("custom".to_string());
//     }
//
//     let ans = Select::new(msg, scope_options).prompt()?;
//
//     if ans == "custom" {
//         let ans = Text::new("Denote the SCOPE of this change:").prompt()?;
//         Ok(Some(ans).filter(|a| !a.is_empty()))
//     } else if ans == "empty" {
//         Ok(None)
//     } else {
//         Ok(Some(ans))
//     }
// }
