use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use uuid::Uuid;
use regex::Regex;
use sqlx::{FromRow, query_as, query};
use std::collections::HashSet;
use chrono::{prelude::*, TimeDelta};

pub mod user;
pub mod task;

pub mod serde_uuid_vec {
    use serde::{self, Serializer, Deserializer, Serialize, Deserialize};
    use uuid::Uuid;

    pub fn serialize<S>(vec: &Vec<Uuid>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let vec_string : Vec<String> = vec.iter()
            .map(|id| id.to_string())
            .collect();
        vec_string.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Uuid>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec_string : Vec<String> = Vec::deserialize(deserializer)?;
        vec_string.iter()
            .map(|id_str| Uuid::parse_str(id_str).map_err(serde::de::Error::custom))
            .collect() 
    }
}




#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MultipleChoiceAnswer {
    pub selected_answers_indices: Vec<u32>
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OpenQuestionAnswer {
    pub content: String,
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AnswerContent {
    MultipleChoice(MultipleChoiceAnswer),
    OpenQuestion(OpenQuestionAnswer)
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AnswerState {
    Solved,
    Unsolved
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Answer {
    #[serde(with = "uuid::serde::simple")]
    pub task_id : Uuid,
    #[serde(with = "uuid::serde::simple")]
    pub user_id : Uuid,
    pub content: Option<AnswerContent>,
    state: AnswerState
}

// Remember to use this, not serde_json::from_str()!
impl Answer {
    pub fn deserialize<'de>(json : &'de str) -> Result<Answer, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn serialize(&self) -> Result<String, serde_json::Error>{
        serde_json::to_string(self)
    }

    pub fn solve(&self, content: AnswerContent) -> Answer {
        Answer {
            user_id: self.user_id,
            task_id: self.task_id,
            content: Some(content),
            state: AnswerState::Solved
        }
    }

    pub fn new(user_id: Uuid, task_id: Uuid) -> Answer {
        Answer {
            task_id,
            user_id,
            content: None,
            state: AnswerState::Unsolved
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_answer_serialization() {
        let original = Answer::new(Uuid::new_v4(), Uuid::new_v4()).solve(
            AnswerContent::OpenQuestion( OpenQuestionAnswer{content: "AAAAAAAAAAA".to_string()})
        );

        let json = Answer::serialize(&original).expect("Couldn't serialize");
        dbg!(&json);
        let deserialized = Answer::deserialize(json.as_str()).expect("Couldn't deserialize");

        assert_eq!(original, deserialized);
    }
    
    
}