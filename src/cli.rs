use clap::{ArgAction, ArgGroup, Args, Parser, Subcommand};

#[derive(Debug)]
pub struct ReviseCommands {
    pub ai: AICommand,
}

#[derive(Debug)]
pub enum AICommand {
    Generate,
    Translate(String),
}

#[derive(Debug, Parser)]
#[clap(
    name = "git-revise",
    about = "A command line utility for better commit"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(name = "ai", about = "Perform AI operations")]
    AI(AIOptions),
}

#[derive(Debug, Args)]
#[clap(group(
    ArgGroup::new("ai")
        .required(true)
        .multiple(false),
))]
struct AIOptions {
    #[clap(short = 'g', long = "generate", action = ArgAction::SetTrue, group = "ai")]
    generate: bool,

    #[clap(short = 't', long = "translate", num_args = 0..=1, group = "ai")]
    translate: Option<String>,
}

impl From<AIOptions> for ReviseCommands {
    fn from(options: AIOptions) -> Self {
        if options.generate {
            Self {
                ai: AICommand::Generate,
            }
        } else {
            Self {
                ai: AICommand::Translate(options.translate.unwrap_or_default()),
            }
        }
    }
}

pub fn parse_command() -> Option<ReviseCommands> {
    let args = Cli::parse();
    args.command
        .map(|Commands::AI(ai_options)| ai_options.into())
}
