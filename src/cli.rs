use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "git-revise")]
// TODO:
#[command(about = "A fictional versioning CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    AI {
        #[arg(short = 'g')]
        generate: bool,
        #[arg(short = 't')]
        translate: bool,
    },
}

pub fn cmd() {
    let args = Cli::parse();

    match args.command {
        Some(c) => {
            println!("{:#?}",c)
            // match c {
                // Commands::AI { generate, translate } => {
                //     if generate {
                //         todo!()
                //     }
                //     if translate {
                //         todo!()
                //     }
                // }
            // }
        },
        None => {
            println!("No subcommand was used");
        }
    }
}


