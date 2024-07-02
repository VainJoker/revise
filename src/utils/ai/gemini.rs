use core::time;

use anyhow::Ok;
use serde::{Deserialize, Serialize};

use crate::{error::ReviseResult};


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub contents: Vec<Content>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub candidates: Vec<Candidate>,
    pub usage_metadata: UsageMetadata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    pub content: Content,
    pub finish_reason: String,
    pub index: i64,
    pub safety_ratings: Vec<SafetyRating>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub parts: Vec<Part>,
    pub role: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Part {
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SafetyRating {
    pub category: String,
    pub probability: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageMetadata {
    pub prompt_token_count: i64,
    pub candidates_token_count: i64,
    pub total_token_count: i64,
}

impl Part {
    pub const fn new(text:String) -> Self {
        Self{
            text,
        }
    }
}


pub struct Gemini{
    pub key: String,
    pub url: String,
    pub prompt: String,
    pub input: String,
}


impl Gemini {
    pub fn new(key: &str, input: &str) -> Self {
        Self { 
            key: key.to_string(), 
            input: input.to_string(),
            url: format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",key),
            prompt: r#"
            # Character
                You're a brilliant coding buddy with top-notch proficiency in Git and GitHub.Your main duty is to assist users in crafting clear and precise Git commit messages.
            ## Skills
            ### Skill 1: Translation Pro
                - Take the user's text and translate it into English, thereby dismantling language hurdles in your journey to refine their commit message procedure.
            ### Skill 2: The Commit Message Maverick
                - Process the git diff given by the user and curate a commit message that confidently and tersely summarizes the changes made. The outcome from both skills should adhere to the following structure: [{"type": "<type>","message": "<message>","body": "<body>"}]
            ## Constraints
                - Commit messages should be between 5-20 words. If the message surpasses this limit, abbreviate it without shedding essential details while employing the 'body' part for detailed elaboration. The message should always commence with a verb.
                - If the user's submission doesn't correspond with the demanded parameters, generate this response: [{"type": "error","message": "Request processing failure","body":"The submitted input isn't compatible with the required parameters"}]
                - Guarantee that all dialogues are carried out in the English language.
                - Present the user with at least three alternative replies for each query.
                - Remain concentrated on tasks strictly linked with creating Git commit messages and avoid straying into conversations outside of this context."#.to_string(),
        }
    }

    pub async fn call(&self) -> ReviseResult<String>{
        let builder = reqwest::Client::builder();
        let client = builder.timeout(time::Duration::from_secs(30)).build()?;
        let request = Request{
            contents: vec![
                Content{ parts: vec![Part::new(self.prompt.clone())], role: "user".to_string() },
                Content{ parts: vec![Part::new(self.input.clone())], role: "user".to_string() }
            ]
        };
        eprintln!("{:#?}",request);
        let result = client.post(&self.url).json(&request).send().await?;
        let response = result.json::<Response>().await?;
        let text = response.candidates.first().unwrap().content.parts.first().unwrap().text.clone();
        Ok(text)
    }
}

#[cfg(test)]
mod tests {
    use tokio::sync::oneshot;

    use crate::config::constant::KEY;

    use super::*;

    #[tokio::test]
    async fn test_gemini_call() {

        let gemini = Gemini::new(
            KEY,
            "翻译: 这是一个测试"
        );

        let (tx,mut rx) = oneshot::channel();

        let task1 = tokio::spawn(async move {
            let spinner = ['|', '/', '-', '\\'];
            let mut idx = 0;
            loop{
                tokio::select! {
                    () = tokio::time::sleep(std::time::Duration::from_millis(100)) => {
                        print!("\rGenerating... {}", spinner[idx]);
                        std::io::Write::flush(&mut std::io::stdout()).unwrap(); // 确保立即打印字符
                        std::thread::sleep(std::time::Duration::from_millis(300));
                        idx = (idx + 1) % spinner.len();
                    }
                    _ = &mut rx => {
                        break;
                    }
                }
            }
        });

        let task2 = gemini.call();

        let result = task2.await.unwrap();

        let _ = tx.send(());

        let _ = task1.await;

        eprintln!("{result}");
    }

}

