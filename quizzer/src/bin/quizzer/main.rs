mod cli;
mod playmode;

use std::io::Write;

use anyhow::Result;
use clap::Parser;
use quizzer::{question::Question, store};

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
    let questions = store::load_questions(path)?;

    if questions.is_empty() {
        println!("No questions found. Use the 'question' subcommand to add questions");
        return Ok(());
    }

    let total = questions.len();
    let mut score = 0usize;

    println!("Starting a quiz with {total} question(s)\n");

    for (i, q) in questions.iter().enumerate() {
        println!("Question {} / {}: {}", i + 1, total, q.question());
        for (j, answer) in q.answers().iter().enumerate() {
            println!("  {}. {answer}", j + 1);
        }

        let answer = loop {
            let s = prompt("Your answer (1-4): ")?;
            match s.parse::<usize>() {
                Ok(n) if (1..=4).contains(&n) => break n - 1,
                _ => println!("Please enter a number between 1 and 4"),
            }
        };

        if q.is_correct(answer) {
            println!("Correct\n");
            score += 1;
        } else {
            println!("Wrong! The correct answer is: {}\n", q.answers()[q.correct_index()]);
        }
    }

    println!("Quiz complete! Final score: {score} / {total}");
    Ok(())
}

fn enter_questions(path: &std::path::Path) -> Result<()> {
    let mut questions: Vec<Question> = if path.exists() {
        store::load_questions(path)?
    } else {
        Vec::new()
    };

    println!("You entered question setting mode\n");

    loop {
        let p = prompt("Enter question (empty to finish): ")?;
        if p.is_empty() {
            break;
        }

        let mut answers: Vec<String> = Vec::with_capacity(4);
        for i in 1..=4usize {
            let a = loop {
                let s = prompt(&format!("\tAnswer {i}: "))?;
                if !s.is_empty() {
                    break s;
                }
                println!("Answer cannot be empty");
            };
            answers.push(a);
        }

        let correct = loop {
            let s = prompt("Enter index of correct answer (1-4): ")?;
            match s.parse::<usize>() {
                Ok(n) if (1..=4).contains(&n) => break n - 1,
                _ => println!("Please enter a number between 1 and 4"),
            }
        };

        questions.push(Question::new(p, answers, correct));
        println!("Question added. Total number of questions: {}\n", questions.len());
    }

    store::save_questions(path, &questions)?;
    println!("Saved {} question to {}", questions.len(), path.display());
    Ok(())
}