use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    question: String,
    answer: Vec<String>,
    correct_answer_index: usize,
}

impl Question {
    pub fn new(question: String, answers: Vec<String>, correct_answer_index: usize) -> Self {
        assert!(correct_answer_index < 4, "correct index must be in 0..4, got {correct_answer_index}");
        assert!(answers.len() == 4,"there must be exactly 4 answers, got {}",answers.len());
        Self {
            question,
            answer: answers,
            correct_answer_index,
        }
    }

    pub fn question(&self) -> &str {
        &self.question
    }

    pub fn answers(&self) -> &Vec<String> {
        &self.answer
    }

    pub fn correct_index(&self) -> usize {
        self.correct_answer_index
    }

    pub fn is_correct(&self, answer: usize) -> bool {
        self.correct_answer_index == answer
    }
}
