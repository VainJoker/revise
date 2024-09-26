use std::{collections::HashMap, path::PathBuf, sync::OnceLock};

use colored::Colorize;
use realme::{Adaptor, EnvParser, EnvSource, FileSource, Realme, TomlParser};
use serde::Deserialize;

use crate::{
    error::ReviseResult,
    git::{repo::GitRepository, GitUtils},
};

pub static CFG: OnceLock<ReviseConfig> = OnceLock::new();

pub fn initialize_config() -> ReviseResult<ReviseConfig> {
    let config = CFG.get_or_init(|| {
        dotenvy::dotenv().ok();
        ReviseConfig::load_config().unwrap_or_else(|e| {
            eprintln!("Load config err: {e}");
            std::process::exit(exitcode::CONFIG);
        })
    });
    Ok(config.clone())
}

pub fn get_config() -> &'static ReviseConfig {
    CFG.get().unwrap()
}

#[derive(Deserialize, Debug, Clone)]
pub struct ReviseConfig {
    pub template: String,
    pub types: Vec<Type>,
    pub emojis: Vec<Emoji>,
    pub scopes: Vec<String>,
    pub auto: Auto,
    #[serde(default)]
    pub api_key: HashMap<String, String>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Render {}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Emoji {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Type {
    pub key: String,
    pub value: String,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Auto {
    pub git: AutoGit,
    pub commit: AutoCommit,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Deserialize, Debug, Clone, Copy, Default)]
pub struct AutoGit {
    pub add: bool,
    pub push: bool,
    pub diff: bool,
    pub footer: bool,
}

#[derive(Deserialize, Debug, Clone, Copy, Default)]
pub struct AutoCommit {
    pub content: bool,
    pub footer: bool,
}

impl ReviseConfig {
    pub fn new() -> Self {
        Self::default()
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

    pub fn get_emoji(&self, key: &str) -> Option<String> {
        self.emojis
            .iter()
            .find(|e| e.key == key)
            .map(|e| e.value.clone())
    }

    pub fn get_scopes(&self) -> Vec<String> {
        self.scopes.clone()
    }

    pub fn get_config_path() -> ReviseResult<Option<PathBuf>> {
        let mut config_paths = Vec::new();

        if let Ok(repo) = GitUtils::git_repo() {
            if let Some(repo_root) = repo.path().parent() {
                config_paths.push(repo_root.join("revise.toml"));
            }
        }

        if let Some(config_dir) = dirs::config_local_dir() {
            config_paths.push(config_dir.join("revise").join("revise.toml"));
        }

        let config_path = config_paths
            .into_iter()
            .find(|path| path.try_exists().unwrap_or(false));

        Ok(config_path)
    }

    pub fn load_config() -> ReviseResult<Self> {
        let config_path = Self::get_config_path()?;
        let config = match config_path {
            Some(path) => {
                return Realme::builder()
                    .load(Adaptor::new(Box::new(EnvSource::<EnvParser>::new(
                        "REVISE_",
                    ))))
                    .load(Adaptor::new(Box::new(
                        FileSource::<TomlParser>::new(path),
                    )))
                    .build()?
                    .try_deserialize()
                    .map_err(|e| anyhow::anyhow!(e.to_string()));
            }
            None => Self::default(),
        };

        let msg = format!(
            "{}",
            "Read config file failed, loading default config!!!!!"
                .red()
                .on_black()
        );
        println!("{msg}");
        Ok(config)
    }
}

#[allow(clippy::too_many_lines)]
impl Default for ReviseConfig {
    fn default() -> Self {
        Self {
            types: vec![
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
            ],
            emojis: vec![
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
                    value: "♻️".to_owned(),
                },
                Emoji {
                    key: "perf".to_owned(),
                    value: "⚡️".to_owned(),
                },
                Emoji {
                    key: "test".to_owned(),
                    value: "✅".to_owned(),
                },
                Emoji {
                    key: "build".to_owned(),
                    value: "📦️".to_owned(),
                },
                Emoji {
                    key: "ci".to_owned(),
                    value: "⚙️".to_owned(),
                },
                Emoji {
                    key: "chore".to_owned(),
                    value: "🔨".to_owned(),
                },
                Emoji {
                    key: "revert".to_owned(),
                    value: "🔙".to_owned(),
                },
            ],
            scopes: Vec::new(),
            auto: Auto {
                git: AutoGit::default(),
                commit: AutoCommit::default(),
            },
            api_key: HashMap::new(),
            template: String::from("
{{commit_icon}} {{ commit_type }}{% if commit_scope %}({{commit_scope}}){% endif %}{% if commit_breaking %}!{% endif %}: {{ commit_subject }}{% if commit_issue %}({{commit_issue}}){% endif %}   
{% if commit_body %}\n{{ commit_body }}{% endif %}
{% if commit_breaking %}\nBREAKING CHANGE: {{ commit_breaking }}{% endif %}"),
        }
    }
}
