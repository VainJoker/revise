use serde::Deserialize;

pub mod gemini;

// ```json\n[\n  {\n    \"type\": \"info\",\n    \"message\": \"Translate commit message\",\n    \"body\": \"Please provide the commit message you want to translate. I'll help you craft a clear and concise message in English.\"\n  }\n]\n``` \n

#[derive(Deserialize, Debug)]
struct Message {
    r#type: String,
    message: String,
    body: String,
}
