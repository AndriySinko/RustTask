use clap::{Parser, Subcommand};
use std::path::PathBuf;
use crate::playmode;


#[derive(Parser)]
struct Cli{
    #[arg(short, long, default_value = "quiz.json")]
    file: PathBuf,

    #[command(subcommand)]
    playmode: playmode::PlayMode,
}


