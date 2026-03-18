use std::path::Path;
use anyhow::{Context, Result};
use crate::question::Question;

pub fn load_questions(path: &Path) -> Result<Vec<Question>> {
    let content = std::fs::read_to_string(path).with_context(|| format!("failed to read '{}'", path.display()))?;
    serde_json::from_str(&content).with_context(|| format!("'{}' contains invalid JSON", path.display()))
}

pub fn save_questions(path: &Path, questions: &[Question]) -> Result<()> {
    let json = serde_json::to_string_pretty(questions)?;
    std::fs::write(path, &json).with_context(|| format!("failed to write '{}'", path.display()))
}
