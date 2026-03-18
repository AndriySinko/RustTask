use clap::{Parser, Subcommand};
use std::path::PathBuf;
use crate::playmode;


#[derive(Parser)]
pub struct Cli{
    #[arg(short, long, default_value = "quiz.json")]
    pub(crate) file: PathBuf,

    #[command(subcommand)]
    pub(crate) playmode: playmode::PlayMode,
}


