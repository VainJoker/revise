use clap::{ArgAction, ArgGroup, Parser};

use crate::config;

#[derive(Debug, Parser)]
#[clap(
    name = "git-revise",
    about = "A command line utility for better commit"
)]
#[clap(version, author, about)]
#[clap(group(
    ArgGroup::new("ai_group")
        .args(&["generate", "translate"])
        .multiple(false)
))]
#[clap(group(
    ArgGroup::new("conflict_group")
        .args(&["message"])
        .conflicts_with_all(&["generate", "translate", "path", "exclude", "include"])
))]
pub struct Cli {
    /// Generate AI-assisted commit message
    #[clap(short = 'g', long = "generate", action = ArgAction::SetTrue)]
    pub generate: bool,

    /// Translate commit message
    #[clap(short = 't', long = "translate", num_args = 0..=1)]
    pub translate: Option<String>,

    /// Add files to staged area
    #[clap(short = 'a', long = "add", num_args = 0.., default_missing_value = ".", value_delimiter = ' ')]
    pub path: Vec<String>,

    /// Exclude files from being added
    #[clap(short = 'x', long = "exclude")]
    pub exclude: Vec<String>,

    /// Include files to be added
    #[clap(short = 'i', long = "include")]
    pub include: Vec<String>,

    /// Specify commit message
    #[clap(short = 'm', long = "message")]
    pub message: Option<String>,
    // /// Revise commit message
    // #[clap(short = 'r', long = "repeat", action = ArgAction::SetTrue)]
    // pub repeat: bool,
}

#[derive(Debug)]
pub struct ReviseCommands {
    pub ai: Option<AICommand>,
    pub add: Vec<String>,
    pub excludes: Vec<String>,
    pub message: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AICommand {
    Generate,
    Translate(String),
}

// #[derive(Debug, Args)]
// pub struct AddOptions {
//     #[clap(name = "path", short = 'a', num_args = 0.., default_missing_value
// = ".", value_delimiter = ' ')]     pub path: Vec<PathBuf>,
//     #[clap(short = 'n', long = "dry-run")]
//     pub dry_run: bool,
//     #[clap(short = 'v', long = "verbose")]
//     pub verbose: bool,
//     #[clap(short, long)]
//     pub update: bool,
// }

pub fn parse_command() -> ReviseCommands {
    let cli = Cli::parse();
    let cfg = config::get_config();

    let mut combined_excludes: Vec<String> = cfg.exclude_files.clone();
    combined_excludes.extend(cli.exclude.clone());

    for path in &cli.include {
        combined_excludes.retain(|item| item != path);
    }

    ReviseCommands {
        ai: if cli.generate {
            Some(AICommand::Generate)
        } else {
            cli.translate.map(AICommand::Translate)
        },
        add: cli.path,
        excludes: combined_excludes,
        message: cli.message,
        // repeat: cli.repeat,
    }
}
