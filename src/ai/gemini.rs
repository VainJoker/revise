use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::AI;
use crate::ReviseResult;

#[derive(Debug, Clone)]
pub struct Gemini {
    prompt: String,
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Commit {
    #[serde(rename = "type")]
    pub kind: String,
    pub message: String,
    pub body: String,
}

impl Gemini {
    pub fn new(key: &str) -> Self {
        let prompt = r#"
        # Character
            You're a brilliant coding buddy with top-notch proficiency in Git. Your main duty is to assist users in crafting clear and precise Git commit messages.

        ## Skills

        ### Skill 1: Multilingual Translation
        - Recognize translation requests in the format: "SourceLanguage:TargetLanguage; Content"
        - Identify requests starting with "这是一个翻译commit" as translation tasks
        - Translate the given content from the source language to the target language
        - Preserve the original text alongside the translation in the output
        - Adapt the translation to fit the context of Git commit messages
        - Example input: "中文:English; 这是一个翻译commit, 优化用户界面布局"
        - Example output: 
          ```json
          [
            {
              "type": "translation",
              "message": "Optimize user interface layout",
              "body": "A long body with details about the changes made"
            },
            {
              "type": "translation",
              "message": "Optimize user interface layout",
              "body": "A long body with details about the changes made"
            },
            {
              "type": "translation",
              "message": "Optimize user interface layout",
              "body": "A long body with details about the changes made"
            }
          ]
          ```

        ### Skill 2: The Commit Message Maverick
        - Process the git diff or description given by the user
        - Curate commit messages that confidently and tersely summarize the changes made
        - Always provide exactly three alternative commit messages for each request
        - Ensure diversity in style and content among the three alternatives

        ## Output Format
        The outcome should adhere to the following structure:
        ```json
        [
          {"type": "<type>", "message": "<message>", "body": "<body>"},
          {"type": "<type>", "message": "<message>", "body": "<body>"},
          {"type": "<type>", "message": "<message>", "body": "<body>"}
        ]
        ```

        ## Constraints
        - Commit messages should be between 5-20 words
        - If the message surpasses this limit, abbreviate it without shedding essential details while employing the 'body' part for detailed elaboration
        - Do not include prefixes like "feat:", "fix:", etc. in the commit message, just put it in <type> part, and start the <message> with a verb
        - Guarantee that all dialogues are carried out in the English language, except for translation requests
        - Remain concentrated on tasks strictly linked with creating Git commit messages
        - Remember to always provide three distinct commit message options.

        ## Error Handling
        If the user's submission doesn't correspond with the demanded parameters, generate this response:
        ```json
        [{"type": "error", "message": "Request processing failure", "body":"The submitted input isn't compatible with the required parameters"}]
        ```

        "#;
        let url = format!(
            "{}/models/{}:{}?key={}",
            "https://generativelanguage.googleapis.com/v1beta",
            "gemini-1.5-pro-latest",
            "generateContent",
            key
        );
        Self {
            prompt: prompt.to_string(),
            url,
        }
    }

    pub async fn call(
        &self,
        input: &str,
    ) -> ReviseResult<HashMap<String, Commit>> {
        let txt_request = Request {
            contents: vec![
                Content {
                    role: Role::User,
                    parts: vec![Part {
                        text: Some(self.prompt.clone()),
                        inline_data: None,
                        file_data: None,
                        video_metadata: None,
                    }],
                },
                Content {
                    role: Role::User,
                    parts: vec![Part {
                        text: Some(input.to_string()),
                        inline_data: None,
                        file_data: None,
                        video_metadata: None,
                    }],
                },
            ],
            tools: vec![],
            safety_settings: vec![],
            generation_config: Some(GenerationConfig {
                temperature: None,
                top_p: None,
                top_k: None,
                candidate_count: None,
                max_output_tokens: None,
                stop_sequences: None,
                response_mime_type: Some("application/json".to_string()),
            }),

            system_instruction: None,
        };

        let client: reqwest::Client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;
        let request_builder = client
            .post(&self.url)
            .header(reqwest::header::USER_AGENT, "crate/revise")
            .header(reqwest::header::CONTENT_TYPE, "application/json");
        let result = request_builder.json(&txt_request).send().await?;
        match result.status() {
            reqwest::StatusCode::OK => {
                let response = result.json::<GeminiResponse>().await?;

                let text = response
                    .candidates
                    .first()
                    .ok_or_else(|| anyhow::anyhow!("No candidates found"))?
                    .content
                    .parts
                    .first()
                    .ok_or_else(|| anyhow::anyhow!("No parts found"))?
                    .text
                    .clone()
                    .ok_or_else(|| anyhow::anyhow!("No text found"))?
                    .clone();
                let messages: Vec<Commit> = serde_json::from_str(&text)?;
                let mut m = HashMap::new();
                for message in messages {
                    let msg = format!("Message: {}", message.message);
                    let body = format!("Body: {}", message.body);
                    m.insert(msg + "\n\r" + &body, message);
                }

                Ok(m)
            }
            _ => Err(anyhow::anyhow!(
                "Failed to get response from Gemini API: {}, response: {}",
                result.status(),
                result.text().await?
            )),
        }
    }
}

impl AI<HashMap<String, Commit>> for Gemini {
    async fn generate_response(
        &self,
        input: &str,
    ) -> ReviseResult<HashMap<String, Commit>> {
        self.call(input).await
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Request {
    pub contents: Vec<Content>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Tools>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default, rename = "safetySettings")]
    pub safety_settings: Vec<SafetySettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, rename = "generationConfig")]
    pub generation_config: Option<GenerationConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default, rename = "system_instruction")]
    pub system_instruction: Option<SystemInstructionContent>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiResponse {
    pub candidates: Vec<Candidate>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Content {
    pub role: Role,
    #[serde(default)]
    pub parts: Vec<Part>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Part {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_data: Option<InlineData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_data: Option<FileData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_metadata: Option<VideoMetadata>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Model,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineData {
    pub mime_type: String,
    pub data: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileData {
    pub mime_type: String,
    pub file_uri: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoMetadata {
    pub start_offset: StartOffset,
    pub end_offset: EndOffset,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StartOffset {
    pub seconds: i32,
    pub nanos: i32,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EndOffset {
    pub seconds: i32,
    pub nanos: i32,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tools {
    #[serde(rename = "functionDeclarations")]
    pub function_declarations: Vec<FunctionDeclaration>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FunctionDeclaration {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SafetySettings {
    pub category: HarmCategory,
    pub threshold: HarmBlockThreshold,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerationConfig {
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub top_k: Option<i32>,
    pub candidate_count: Option<i32>,
    pub max_output_tokens: Option<i32>,
    pub stop_sequences: Option<Vec<String>>,
    pub response_mime_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SystemInstructionContent {
    #[serde(default)]
    pub parts: Vec<SystemInstructionPart>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInstructionPart {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candidate {
    pub content: Content,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmCategory {
    HarmCategorySexuallyExplicit,
    HarmCategoryHateSpeech,
    HarmCategoryHarassment,
    HarmCategoryDangerousContent,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmProbability {
    HarmProbabilityUnspecified,
    Negligible,
    Low,
    Medium,
    High,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HarmBlockThreshold {
    BlockNone,
    BlockLowAndAbove,
    BlockMedAndAbove,
    BlockHighAndAbove,
}

#[cfg(test)]
mod tests {
    use tokio::sync::oneshot;

    use super::*;

    #[ignore]
    #[tokio::test]
    #[allow(clippy::needless_return)]
    async fn test_gemini_call() {
        dotenvy::dotenv().ok();
        let key = std::env::var("REVISE_GEMINI_KEY").unwrap();
        let gemini = Gemini::new(&key);

        let (tx, mut rx) = oneshot::channel();

        let task1 = tokio::spawn(async move {
            let spinner = ['|', '/', '-', '\\'];
            let mut idx = 0;
            loop {
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

        let task2 = gemini.call("翻译: 这是一个测试");

        let result = task2.await.unwrap();

        let _ = tx.send(());

        let _ = task1.await;

        eprintln!("{result:#?}");
    }
}
