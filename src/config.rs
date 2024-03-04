use std::{env, path::Path};

use serde::Deserialize;

use crate::error::ReviseResult;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub messages: Vec<Message>,
    #[serde(rename = "emoji")]
    pub emojis: Vec<Emoji>,
    #[serde(rename = "emojiAlign")]
    pub emoji_align: String,
    pub scopes: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Message {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Emoji {
    pub key: String,
    pub value: String,
}

pub fn toml_parser(path: &Path) -> ReviseResult<Config> {
    Ok(toml::from_str(&std::fs::read_to_string(path)?)?)
}

pub fn load_config() -> ReviseResult<Config> {
    let mut current_path = env::current_dir()?;
    current_path.push("revise.toml");
    if let Ok(true) = current_path.try_exists() {
        return toml_parser(&current_path);
    } else if let Some(mut config_path) = dirs::config_local_dir() {
        config_path.push("revise");
        config_path.push("revise.toml");
        if let Ok(true) = config_path.try_exists() {
            return toml_parser(&current_path);
        }
    }
    Ok(Config::default())
}

#[test]
fn test_toml_parser() {
    let res = toml_parser(Path::new("./revise.toml"));
    assert!(res.is_ok())
}

impl Default for Config {
    fn default() -> Self {
        Config {
            messages: [
                Message {
                    key: "type".to_owned(),
                    value: "Select the type of change that you\"re committing=".to_owned(),
                },
                Message {
                    key: "scope".to_owned(),
                    value: "Denote the SCOPE of this change (optional)=".to_owned(),
                },
                Message {
                    key: "customScope".to_owned(),
                    value: "Denote the SCOPE of this change=".to_owned(),
                },
                Message {
                    key: "subject".to_owned(),
                    value: "Write a SHORT, IMPERATIVE tense description of the change=\n".to_owned(),
                },
                Message {
                    key: "body".to_owned(),
                    value: "Provide a LONGER description of the change (optional). Use '|' to break new line=\n".to_owned(),
                },
                Message {
                    key: "breaking".to_owned(),
                    value: "List any BREAKING CHANGES (optional). Use '|' to break new line=\n".to_owned(),
                },
                Message {
                    key: "footerPrefixesSelect".to_owned(),
                    value: "Select the ISSUES type of changeList by this change (optional)=".to_owned(),
                },
                Message {
                    key: "customFooterPrefix".to_owned(),
                    value: "Input ISSUES prefix=".to_owned(),
                },
                Message {
                    key: "footer".to_owned(),
                    value: "List any ISSUES by this change. E.g.= #31, #34=\n".to_owned(),
                },
                Message {
                    key: "generatingByAI".to_owned(),
                    value: "Generating your AI commit subject...".to_owned(),
                },
                Message {
                    key: "generatedSelectByAI".to_owned(),
                    value: "Select suitable subject by AI generated=".to_owned(),
                },
                Message {
                    key: "confirmCommit".to_owned(),
                    value: "Are you sure you want to proceed with the commit above?".to_owned(),
                },
                ].to_vec(),
                emojis: [
                    Emoji {
                        key: "feat".to_owned(),
                        value: "✨".to_owned(),
                    },
                    Emoji {
                        key: "fix".to_owned(),
                        value: "🐛".to_owned(),
                    },
                    Emoji {
                        key: "docs".to_owned(),
                        value: "📚".to_owned(),
                    },
                    Emoji {
                        key: "style".to_owned(),
                        value: "🎨".to_owned(),
                    },
                    Emoji {
                        key: "refactor".to_owned(),
                        value: "♻\u{fe0f}".to_owned(),
                    },
                    Emoji {
                        key: "perf".to_owned(),
                        value: "⚡\u{fe0f}".to_owned(),
                    },
                    Emoji {
                        key: "test".to_owned(),
                        value: "✅".to_owned(),
                    },
                    Emoji {
                        key: "build".to_owned(),
                        value: "📦\u{fe0f}".to_owned(),
                    },
                    Emoji {
                        key: "ci".to_owned(),
                        value: "⚙\u{fe0f}".to_owned(),
                    },
                    Emoji {
                        key: "chore".to_owned(),
                        value: "🔨".to_owned(),
                    },
                    Emoji {
                        key: "revert".to_owned(),
                        value: "◀\u{fe0f}".to_owned(),
                    },
                    ].to_vec(),
                    emoji_align: "center".to_string(),
                    scopes: Vec::new()
        }
    }
}
