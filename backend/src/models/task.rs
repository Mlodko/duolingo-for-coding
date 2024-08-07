use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use uuid::Uuid;
use sqlx::query;
use std::collections::HashSet;
use chrono::{prelude::*, TimeDelta};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct OpenQuestionTask {
    pub content: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MultipleChoiceTask {
    pub choices: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum TaskContent {
    Open(OpenQuestionTask),
    MultipleChoice(MultipleChoiceTask)
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct Tag {
    pub id: Uuid,
    pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    #[serde(with = "uuid::serde::simple")]
    pub id: Uuid,
    pub title: String,
    pub content: TaskContent,
    pub tags: HashSet<Tag>
}

impl Task {
    pub fn new(title : String, content : TaskContent, tags: HashSet<Tag>) -> Task {
        Task {
            id : uuid::Uuid::new_v4(),
            title,
            content,
            tags
        }
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
        self.title == other.title &&
        self.content == other.content &&
        self.tags == other.tags
    }
}

impl Tag {
    pub async fn new(name: String, pool: &MySqlPool) -> Tag {
        let existing_tag = query!("SELECT id FROM tags WHERE name = ?",
            name
        ).fetch_optional(pool).await.expect("Couldn't query db for existing tags");
        
        if let Some(tag) = existing_tag {
            return Tag {
                id: Uuid::parse_str(&tag.id).unwrap(),
                name
            }
        }
        
        Tag {
            id: uuid::Uuid::new_v4(),
            name
        }
    }
}




pub mod database {
    use super::*;
    impl Task {
        pub async fn insert(&self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
            let content_json = serde_json::to_string(&self.content).expect("Couldn't serialize content");
            let mut transaction = pool.begin().await?;
            // Insert the task into the database
            sqlx::query!(
                r#"INSERT INTO tasks (id, title, content) VALUES (?, ?, ?)"#,
                self.id.to_string(),
                self.title,
                content_json
            )
            .execute(&mut *transaction)
            .await?;
            
            // Insert the tags into the database
            for tag in &self.tags {
                // Check if tag already exists
                let tag_id = match sqlx::query!(
                    "SELECT id FROM tags WHERE name = ?",
                    tag.name
                )
                .fetch_optional(pool)
                .await? {
                    Some(record) => Uuid::parse_str(&record.id).unwrap(),
                    None => {
                        // Insert the tag into the database
                        sqlx::query!(
                            "INSERT INTO tags (id, name) VALUES (?, ?)",
                            tag.id.to_string(),
                            tag.name
                        )
                        .execute(&mut *transaction)
                        .await?;
                        tag.id
                    }
                };
                
                // Insert the task-tag relation into the database
                sqlx::query!(
                    "INSERT INTO task_tags (task_id, tag_id) VALUES (?, ?)",
                    self.id.to_string(),
                    tag_id.to_string()
                )
                .execute(&mut *transaction)
                .await?;
            }
            
            transaction.commit().await?;
            
            Ok(())
        }
        
        pub async fn read(id: Uuid, pool: &MySqlPool) -> Result<Task, sqlx::Error> {
            // Read the row from the tasks table
            let task_row = query!(
                "SELECT * FROM tasks WHERE id = ?",
                id.to_string()
            ).fetch_one(pool).await?;
            
            // Search for tags by tag_ids in the tags table
            let tag_rows = query!(
                "SELECT id, name FROM tags WHERE id IN (SELECT tag_id FROM task_tags WHERE task_id = ?)",
                id.to_string()
            ).fetch_all(pool).await?;
            
            dbg!(&tag_rows);
            
            let tags = tag_rows
                .iter()
                .map(|tag_row| Tag {
                    id: Uuid::parse_str(&tag_row.id).unwrap(),
                    name: tag_row.name.clone()
                })
                .collect();
            
            dbg!(&tags);
            
            
            Ok(Task {
                id: Uuid::parse_str(&task_row.id).unwrap(),
                title: task_row.title,
                content: serde_json::from_value(task_row.content).unwrap(),
                tags
            })
        }
        
        pub async fn delete(id: Uuid, pool: &MySqlPool) -> Result<(), sqlx::Error> {
            let mut transaction = pool.begin().await?;
            
            // Remove the task from the task_tags table
            query!("DELETE FROM task_tags WHERE task_id = ?",
                &id.to_string())
            .execute(&mut *transaction).await?;
            
            // Remove the task from the tasks table
            query!("DELETE FROM tasks WHERE id = ?",
                &id.to_string())
            .execute(&mut *transaction).await?;
            
            transaction.commit().await
        }
        
        async fn update(&self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
            let db_task = Task::read(self.id, pool).await?;
            
            if self == &db_task {
                return Ok(());
            }
            
            let mut transaction = pool.begin().await?;
            
            // Update task table
            query!(
                "UPDATE tasks SET title = ?, content = ? WHERE id = ?",
                self.title,
                serde_json::to_value(&self.content).unwrap(),
                self.id.to_string()
            ).execute(&mut *transaction).await?;
            
            // Check if tags have been added or removed
            if db_task.tags == self.tags {
                return Ok(());
            }
            
            // Remove unneeded tags from task_tags
            let tags_to_delete = &db_task.tags
                .difference(&self.tags)
                .collect::<Vec<&Tag>>();
            
            for tag in tags_to_delete {
                query!(
                    "DELETE FROM task_tags WHERE task_id = ? AND tag_id = ?",
                    self.id.to_string(),
                    tag.id.to_string()
                ).execute(&mut *transaction).await?;
            }
            
            // Add new tags to task_tags and tags if needed
            
            let tasks_to_add = &self.tags
                .difference(&db_task.tags)
                .collect::<Vec<&Tag>>();
            
            for tag in tasks_to_add {
                // Check if tag already exists
                let tag_id = match sqlx::query!(
                    "SELECT id FROM tags WHERE name = ?",
                    tag.name
                )
                .fetch_optional(pool)
                .await? {
                    Some(record) => Uuid::parse_str(&record.id).unwrap(),
                    None => {
                        // Insert the tag into the database
                        sqlx::query!(
                            "INSERT INTO tags (id, name) VALUES (?, ?)",
                            tag.id.to_string(),
                            tag.name
                        )
                        .execute(&mut *transaction)
                        .await?;
                        tag.id
                    }
                };
                
                // Insert the task-tag relation into the database
                sqlx::query!(
                    "INSERT INTO task_tags (task_id, tag_id) VALUES (?, ?)",
                    self.id.to_string(),
                    tag_id.to_string()
                )
                .execute(&mut *transaction)
                .await?;
            }
            
            transaction.commit().await?;
            
            Ok(())
        }
    }
}

pub mod json {
    use super::Task;
    
    impl Task {
        pub fn serialize(&self) -> Result<String, serde_json::Error> {
            serde_json::to_string(self)
        }
    
        pub fn deserialize(json: &str) -> Result<Task, serde_json::Error> {
            serde_json::from_str(json)
        }
    }
}

mod tests {
    use super::*;
    use crate::database;
    
    #[tokio::test]
    async fn test_task_db_insertion() {
        let binding = database::get_database_connection_pool().await.unwrap();
        let pool = binding.lock().await;
        let content = OpenQuestionTask { content: "Code an AGI. You have 2 minutes and cannot use google".to_string() };
        let tags = HashSet::from([Tag::new("AI".to_string(), &pool).await, Tag::new("AGI".to_string(), &pool).await]);
        let task = Task::new("Test task".to_string(), TaskContent::Open(content), tags);
        
        let result = task.insert(&pool).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_task_db_read() {
        let binding = database::get_database_connection_pool().await.unwrap();
        let pool = binding.lock().await;
        let content = OpenQuestionTask { content: "Code an AGI. You have 2 minutes and cannot use google".to_string() };
        let tags = HashSet::from([Tag::new("AI".to_string(), &pool).await, Tag::new("AGI".to_string(), &pool).await]);
        let task = Task::new("Test task".to_string(), TaskContent::Open(content), tags);
        
        assert!(task.insert(&pool).await.is_ok());
        
        let read_task = Task::read(task.id, &pool).await;      
      
        if let Err(e) = Task::delete(task.id, &pool).await {
            dbg!("Couldn't cleanup after the opertaion");
            dbg!(e);
        }
        
        dbg!("{:#?}", &read_task);
        
        assert!(read_task.is_ok());
        
        assert_eq!(task, read_task.unwrap());  
    }

    
    #[tokio::test]
    async fn test_task_db_deletion() {
        let binding = database::get_database_connection_pool().await.unwrap();
        let pool = binding.lock().await;
        let content = OpenQuestionTask { content: "Code an AGI. You have 2 minutes and cannot use google".to_string() };
        let tags = HashSet::from([Tag::new("AI".to_string(), &pool).await, Tag::new("AGI".to_string(), &pool).await]);
        let task = Task::new("Test task".to_string(), TaskContent::Open(content), tags);
        
        task.insert(&pool).await;
        
        let result = Task::delete(task.id, &pool).await;
        assert_eq!(result.is_ok(), true);
    }
    
    #[tokio::test]
    async fn test_task_serialization() {
        let task = Task::new("Test task".to_string(), TaskContent::Open(OpenQuestionTask { content: "Code an AGI. You have 2 minutes and cannot use google".to_string() }),
            HashSet::from([Tag{id: Uuid::new_v4(), name: "AI".to_string()}, Tag{id: Uuid::new_v4(), name: "Programming".to_string()}]));
        let serialized = task.serialize().unwrap();
        let deserialized = Task::deserialize(&serialized).unwrap();
        assert_eq!(task, deserialized);
    }
}
