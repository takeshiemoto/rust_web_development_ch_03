use crate::types::answer::{Answer, AnswerId};
use crate::types::question::{Question, QuestionId};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct Store {
    pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    pub answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Self::init())),
            answers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../question.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }
}
