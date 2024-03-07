use std::{env, path::Path};

use serde::Deserialize;

use crate::error::ReviseResult;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub messages: Vec<Message>,
    pub types: Vec<Type>,
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

impl Config {
    pub fn get_types(&self) -> Vec<String> {
        let types = self.types.clone();
        types.into_iter().map(|t| t.key + &t.value).collect()
    }

    pub fn get_scopes(&self) -> Vec<String> {
        self.scopes.clone()
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
}

pub fn toml_parser(path: &Path) -> ReviseResult<Config> {
    Ok(toml::from_str(&std::fs::read_to_string(path)?)?)
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
                types: [
                    Type { key: "feat:     ".to_owned(),value: "A new feature".to_owned() },
                    Type { key: "fix:      ".to_owned(), value: "A bug fix".to_owned()},
                    Type { key: "docs:     ".to_owned(), value: "Documentation only changes".to_owned()},
                    Type { key: "style:    ".to_owned(), value: "Changes that do not affect the meaning of the code".to_owned()},
                    Type { key: "refactor: ".to_owned(), value: "A code change that neither fixes a bug nor adds a feature".to_owned()},
                    Type { key: "perf:     ".to_owned(), value: "A code change that improves performance".to_owned()},
                    Type { key: "test:     ".to_owned(), value: "Adding missing tests or correcting existing tests".to_owned()},
                    Type { key: "build:    ".to_owned(), value: "Changes that affect the build system or external dependencies".to_owned()},
                    Type { key: "ci:       ".to_owned(), value: "Changes to our CI configuration files and scripts".to_owned()},
                    Type { key: "chore:    ".to_owned(), value: "Other changes that don\"t modify src or test files".to_owned()},
                    Type { key: "revert:   ".to_owned(), value: "Reverts a previous commit".to_owned()}
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
