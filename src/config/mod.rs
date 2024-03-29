use std::sync::OnceLock;

use colored::Colorize;
use serde::Deserialize;

use crate::{error::ReviseResult, utils::git::GitUtils};

pub static CONFIG: OnceLock<ReviseConfig> = OnceLock::new();

pub fn initialize_config() -> ReviseResult<ReviseConfig> {
    let config = CONFIG.get_or_init(|| {
        ReviseConfig::load_config().unwrap_or_else(|e| {
            eprintln!("Load config err: {e}");
            std::process::exit(exitcode::CONFIG);
        })
    });
    Ok(config.clone())
}

pub fn get_config() -> &'static ReviseConfig {
    CONFIG.get().unwrap()
}

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
pub struct ReviseRenderConfig {}

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
            types: Vec::new(),
            emojis: Vec::new(),
            emoji_align: String::new(),
            scopes: Vec::new(),
        }
    }

    pub fn get_types(&self) -> Vec<String> {
        let types = self.types.clone();
        let max_key_len = types.iter().map(|t| t.key.len()).max().unwrap_or(5);
        types
            .into_iter()
            .map(|t| {
                let padding = " ".repeat(max_key_len - t.key.len() + 1);
                format!("{}:{}{}", t.key, padding, t.value)
            })
            .collect()
    }

    pub fn get_type_key(&self, idx: usize) -> Option<String> {
        let types = self.types.clone();
        if let Some(t) = types.get(idx) {
            return Some(t.key.clone());
        }
        None
    }

    pub fn get_scopes(&self) -> Vec<String> {
        self.scopes.clone()
    }

    pub fn load_config() -> ReviseResult<Self> {
        let mut current_path = GitUtils::git_repository()?;
        current_path.push("revise.toml");
        if matches!(current_path.try_exists(), Ok(true)) {
            return Ok(toml::from_str(&std::fs::read_to_string(
                &current_path,
            )?)?);
        }
        if let Some(mut config_path) = dirs::config_local_dir() {
            config_path.push("revise");
            config_path.push("revise.toml");
            if matches!(config_path.try_exists(), Ok(true)) {
                return Ok(toml::from_str(&std::fs::read_to_string(
                    &current_path,
                )?)?);
            }
        }
        let msg = format!(
            "{}",
            "Read config file failed, loading default config!!!!!"
                .red()
                .on_black()
        );
        println!("{msg}");
        Ok(Self::default())
    }
}

impl Default for ReviseConfig {
    fn default() -> Self {
        Self {
            types: [
                Type {
                    key: "feat".to_owned(),
                    value: "A new feature".to_owned(),
                },
                Type {
                    key: "fix".to_owned(),
                    value: "A bug fix".to_owned(),
                },
                Type {
                    key: "docs".to_owned(),
                    value: "Documentation only changes".to_owned(),
                },
                Type {
                    key: "style".to_owned(),
                    value: "Changes that do not affect the meaning of the code".to_owned(),
                },
                Type {
                    key: "refactor".to_owned(),
                    value: "A code change that neither fixes a bug nor adds a feature".to_owned(),
                },
                Type {
                    key: "perf".to_owned(),
                    value: "A code change that improves performance".to_owned(),
                },
                Type {
                    key: "test".to_owned(),
                    value: "Adding missing tests or correcting existing tests".to_owned(),
                },
                Type {
                    key: "build".to_owned(),
                    value: "Changes that affect the build system or external dependencies"
                        .to_owned(),
                },
                Type {
                    key: "ci".to_owned(),
                    value: "Changes to our CI configuration files and scripts".to_owned(),
                },
                Type {
                    key: "chore".to_owned(),
                    value: "Other changes that don\"t modify src or test files".to_owned(),
                },
                Type {
                    key: "revert".to_owned(),
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
