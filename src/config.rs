use std::{env, path::Path};

use serde::Deserialize;

use crate::error::ReviseResult;

#[derive(Deserialize, Debug, Clone)]
pub struct ReviseConfig {
    pub types: Vec<Type>,
    #[serde(rename = "emoji")]
    pub emojis: Vec<Emoji>,
    #[serde(rename = "emojiAlign")]
    pub emoji_align: String,
    pub scopes: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Emoji {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Type {
    pub key: String,
    pub value: String,
}

impl Type {
    pub fn get_type(&self) -> String {
        format!("{}{}", self.key, self.value)
    }
}

impl ReviseConfig {
    pub fn new() -> Self {
        Self {
            // messages: Vec::new(),
            types: Vec::new(),
            emojis: Vec::new(),
            emoji_align: String::new(),
            scopes: Vec::new(),
        }
    }
    pub fn get_types(&self) -> Vec<String> {
        let types = self.types.clone();
        types.into_iter().map(|t| t.key + &t.value).collect()
    }

    pub fn get_scopes(&self) -> Vec<String> {
        self.scopes.clone()
    }
    pub fn load_config() -> ReviseResult<ReviseConfig> {
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
        Ok(ReviseConfig::default())
    }
}

pub fn toml_parser(path: &Path) -> ReviseResult<ReviseConfig> {
    Ok(toml::from_str(&std::fs::read_to_string(path)?)?)
}

#[test]
fn test_toml_parser() {
    let res = toml_parser(Path::new("./revise.toml"));
    assert!(res.is_ok())
}

impl Default for ReviseConfig {
    fn default() -> Self {
        ReviseConfig {
            types: [
                Type {
                    key: "feat:     ".to_owned(),
                    value: "A new feature".to_owned(),
                },
                Type {
                    key: "fix:      ".to_owned(),
                    value: "A bug fix".to_owned(),
                },
                Type {
                    key: "docs:     ".to_owned(),
                    value: "Documentation only changes".to_owned(),
                },
                Type {
                    key: "style:    ".to_owned(),
                    value: "Changes that do not affect the meaning of the code".to_owned(),
                },
                Type {
                    key: "refactor: ".to_owned(),
                    value: "A code change that neither fixes a bug nor adds a feature".to_owned(),
                },
                Type {
                    key: "perf:     ".to_owned(),
                    value: "A code change that improves performance".to_owned(),
                },
                Type {
                    key: "test:     ".to_owned(),
                    value: "Adding missing tests or correcting existing tests".to_owned(),
                },
                Type {
                    key: "build:    ".to_owned(),
                    value: "Changes that affect the build system or external dependencies"
                        .to_owned(),
                },
                Type {
                    key: "ci:       ".to_owned(),
                    value: "Changes to our CI configuration files and scripts".to_owned(),
                },
                Type {
                    key: "chore:    ".to_owned(),
                    value: "Other changes that don\"t modify src or test files".to_owned(),
                },
                Type {
                    key: "revert:   ".to_owned(),
                    value: "Reverts a previous commit".to_owned(),
                },
            ]
            .to_vec(),
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
            ]
            .to_vec(),
            emoji_align: "center".to_string(),
            scopes: Vec::new(),
        }
    }
}
