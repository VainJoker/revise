use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::revise::template::CommitTemplate;

#[derive(Debug, Parser, Clone)]
pub struct ReviseArgs {
    #[arg(short = 'c')]
    pub config: Option<PathBuf>,
    #[arg(short = 'm')]
    pub message: Option<String>,
    #[command(subcommand)]
    pub command: Option<ReviseCommand>,
}

#[derive(Debug, Subcommand, Clone)]
pub enum ReviseCommand {
    Commit(CommitTemplate),
}
