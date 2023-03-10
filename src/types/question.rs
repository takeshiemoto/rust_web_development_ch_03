use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct QuestionId(pub i32);
