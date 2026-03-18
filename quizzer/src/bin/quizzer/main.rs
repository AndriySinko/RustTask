mod cli;
mod playmode;

use std::io::Write;
use std::path::PathBuf;

use quizzer::{question::Question, store};
use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};

fn prompt(msg: &str) -> Result<String> {
    print!("{msg}");
    std::io::stdout().flush()?;
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().to_string())
}

fn main() -> Result<()>{

    let cli = cli::Cli::parse();
    match cli.playmode {
        playmode::PlayMode::Question => enter_questions(&cli.file),
        playmode::PlayMode::Quiz => run_quiz(&cli.file),
    }

}

fn run_quiz(path: &std::path::Path) -> Result<()> {

}

fn enter_questions(path: &std::path::Path) -> Result<()> {
    let mut questions: Vec<Question> = if path.exists() {
        store::load_questions(path)?
    } else {
        Vec::new()
    };

    println!("You entered question setting mode. Leave space blank to exit this mode.\n");

    loop {
        let p = prompt("Enter question (empty to finish): ")?;
        if p.is_empty() {
            break;
        }

        let mut answers: Vec<String> = Vec::with_capacity(4);
        for i in 1..=4usize {
            let a = loop {
                let s = prompt(&format!("  Answer {i}: "))?;
                if !s.is_empty() {
                    break s;
                }
                println!("Answer cannot be empty.");
            };
            answers.push(a);
        }

        let correct = loop {
            let s = prompt("Enter index of correct answer (1-4): ")?;
            match s.parse::<usize>() {
                Ok(n) if (1..=4).contains(&n) => break n - 1,
                _ => println!("Please enter a number between 1 and 4."),
            }
        };

        questions.push(Question::new(p, answers, correct));
        println!("Question added. Total number of questions: {}\n", questions.len());
    }

    store::save_questions(path, &questions)?;
    println!(
        "Saved {} question to {}.",
        questions.len(),
        path.display()
    );
    Ok(())
}