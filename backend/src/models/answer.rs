use serde::{Serialize, Deserialize};
use uuid::Uuid;

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
pub struct Answer {
    #[serde(with="uuid::serde::simple")]
    pub id : Uuid,
    #[serde(with = "uuid::serde::simple")]
    pub task_id : Uuid,
    #[serde(with = "uuid::serde::simple")]
    pub user_id : Uuid,
    pub content: Option<AnswerContent>,
}

// Remember to use this, not serde_json::from_str()!
impl Answer {
    

    pub fn solve(&self, content: AnswerContent) -> Answer {
        Answer {
            id: self.id,
            user_id: self.user_id,
            task_id: self.task_id,
            content: Some(content),
        }
    }

    pub fn new(user_id: Uuid, task_id: Uuid) -> Answer {
        Answer {
            id: Uuid::new_v4(),
            task_id,
            user_id,
            content: None,
        }
    }
}

pub mod json {
    use super::*;
    impl Answer {
        pub fn deserialize(json : &str) -> Result<Answer, serde_json::Error> {
            serde_json::from_str(json)
        }
    
        pub fn serialize(&self) -> Result<String, serde_json::Error>{
            serde_json::to_string(self)
        }
    }
}

pub mod database {
    use super::*;
    use sqlx::{query, MySql, MySqlPool, Transaction};
    
    impl Answer {
        pub async fn create(&self, transaction: &mut Transaction<'static, MySql>) -> Result<(), sqlx::Error> {
            query!(
                "INSERT INTO answers (id, task_id, user_id, content) VALUES (?, ?, ?, ?)",
                self.id.to_string(),
                self.task_id.to_string(),
                self.user_id.to_string(),
                serde_json::to_string(&self.content).expect("Couldn't serialize content")
            ).execute(transaction.as_mut()).await?;
            Ok(())
        }
        
        pub async fn read(id: Uuid, transaction: &mut Transaction<'static, MySql>) -> Result<Option<Answer>, sqlx::Error> {
            let row = query!(
                "SELECT * FROM answers WHERE id = ?",
                id.to_string()
            ).fetch_optional(transaction.as_mut()).await?;
            
            match row {
                Some(row) => Ok(Some(Answer {
                    id: Uuid::parse_str(&row.id).expect("Couldn't parse id"),
                    task_id: Uuid::parse_str(&row.task_id).expect("Couldn't parse task_id"),
                    user_id: Uuid::parse_str(&row.user_id).expect("Couldn't parse user_id"),
                    content: row.content.map(|content| serde_json::from_value(content).expect("Couldn't deserialize content"))
                })),
                None => Ok(None)
            }
        }
        
        pub async fn update(&self, transaction: &mut Transaction<'static, MySql>) -> Result<(), sqlx::Error> {
            query!(
                "UPDATE answers SET task_id = ?, user_id = ?, content = ? WHERE id = ?",
                self.task_id.to_string(),
                self.user_id.to_string(),
                serde_json::to_string(&self.content).expect("Couldn't serialize content"),
                self.id.to_string()
            ).execute(transaction.as_mut()).await?;
            Ok(())
        }
        
        pub async fn delete(id: Uuid, transaction: &mut Transaction<'static, MySql>) -> Result<(), sqlx::Error> {
            query!(
                "DELETE FROM answers WHERE id = ?",
                id.to_string()
            ).execute(transaction.as_mut()).await?;
            
            Ok(())
        }
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[tokio::test]
        async fn test_create() {
            let binding = crate::database::get_database_connection_pool().await.expect("Couldn't get pool");
            let pool = binding.lock().await;
            let mut transaction = pool.begin().await.expect("Couldn't begin transaction");
            
            let answer = Answer::new(Uuid::new_v4(), Uuid::new_v4()).solve(
                AnswerContent::OpenQuestion( OpenQuestionAnswer{content: "AAAAAAAAAAA".to_string()})
            );
            
            answer.create(&mut transaction).await.expect("Couldn't create");
            
            transaction.rollback().await.expect("Couldn't rollback");
        }
        
        #[tokio::test]
        async fn test_read() {
            let binding = crate::database::get_database_connection_pool().await.expect("Couldn't get pool");
            let pool = binding.lock().await;
            let mut transaction = pool.begin().await.expect("Couldn't begin transaction");
            
            let answer = Answer::new(Uuid::new_v4(), Uuid::new_v4()).solve(
                AnswerContent::OpenQuestion( OpenQuestionAnswer{content: "AAAAAAAAAAA".to_string()})
            );
            
            answer.create(&mut transaction).await.expect("Couldn't create");
            
            let read_answer = Answer::read(answer.id, &mut transaction).await.expect("Couldn't read").expect("No answer found");
            
            assert_eq!(answer, read_answer);
            
            transaction.rollback().await.expect("Couldn't rollback");
        }
        
        #[tokio::test]
        async fn test_update() {
            let binding = crate::database::get_database_connection_pool().await.expect("Couldn't get pool");
            let pool = binding.lock().await;
            let mut transaction = pool.begin().await.expect("Couldn't begin transaction");
            
            let mut answer = Answer::new(Uuid::new_v4(), Uuid::new_v4()).solve(
                AnswerContent::OpenQuestion( OpenQuestionAnswer{content: "AAAAAAAAAAA".to_string()})
            );
            
            answer.create(&mut transaction).await.expect("Couldn't create");
            
            answer.content = Some(AnswerContent::OpenQuestion( OpenQuestionAnswer{content: "BBBBBBBBBBB".to_string()}));
            
            answer.update(&mut transaction).await.expect("Couldn't update");
            
            let read_answer = Answer::read(answer.id, &mut transaction).await.expect("Couldn't read").expect("No answer found");
            
            assert_eq!(answer, read_answer);
            
            transaction.rollback().await.expect("Couldn't rollback");
        }
        
        #[tokio::test]
        async fn test_delete() {
            let binding = crate::database::get_database_connection_pool().await.expect("Couldn't get pool");
            let pool = binding.lock().await;
            let mut transaction = pool.begin().await.expect("Couldn't begin transaction");
            
            let answer = Answer::new(Uuid::new_v4(), Uuid::new_v4()).solve(
                AnswerContent::OpenQuestion( OpenQuestionAnswer{content: "AAAAAAAAAAA".to_string()})
            );
            
            answer.create(&mut transaction).await.expect("Couldn't create");
            
            Answer::delete(answer.id, &mut transaction).await.expect("Couldn't delete");
            
            let read_answer = Answer::read(answer.id, &mut transaction).await.expect("Couldn't read");
            
            assert!(read_answer.is_none());
            
            transaction.rollback().await.expect("Couldn't rollback");
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