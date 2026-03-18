pub struct Question {
    qustion: String,
    answers: Vec<Answer>,
}

pub struct Answer {
    answer: String,
    is_correct: bool,
}