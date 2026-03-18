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

fn main() {

    let cli = cli::Cli::parse();
    match cli.playmode {
        playmode::PlayMode::Question => enter_questions(&cli.file),
    }

}

fn enter_questions(path: &std::path::Path) -> Result<()> {
    // Load existing questions so new ones are appended rather than replaced.
    let mut questions: Vec<Question> = if path.exists() {
        store::load_questions(path)?
    } else {
        Vec::new()
    };

    println!("Question-entering mode. Leave the prompt blank to stop.\n");

    loop {
        let p = prompt("Question prompt (empty to finish): ")?;
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
                println!("  Answer cannot be empty.");
            };
            answers.push(a);
        }

        // let answers: [String; 4] = answers
        //     .try_into()
        //     .map_err(|_| anyhow!("internal error: failed to collect 4 answers"))?;

        let correct = loop {
            let s = prompt("  Which answer is correct? (1-4): ")?;
            match s.parse::<usize>() {
                Ok(n) if (1..=4).contains(&n) => break n - 1,
                _ => println!("  Please enter a number between 1 and 4."),
            }
        };

        questions.push(Question::new(p, answers, correct));
        println!("  Question added. Total so far: {}\n", questions.len());
    }

    store::save_questions(path, &questions)?;
    println!(
        "Saved {} question(s) to {}.",
        questions.len(),
        path.display()
    );
    Ok(())
}