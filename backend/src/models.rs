use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use uuid::Uuid;
use regex::Regex;
use sqlx::{FromRow, query_as, query};
use std::collections::HashSet;

mod serde_uuid_vec {
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

impl Task {
    pub async fn insert(&self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
        let existing_task = query!("SELECT id FROM tasks WHERE id = ?",
            self.id.to_string()
        ).fetch_optional(pool).await?;
        
        
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
    
    async fn delete(id: Uuid, pool: &MySqlPool) -> Result<(), sqlx::Error> {
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
        if &db_task.tags == &self.tags {
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
    
    pub fn new(title : String, content : TaskContent, tags: HashSet<Tag>) -> Task {
        Task {
            id : uuid::Uuid::new_v4(),
            title,
            content,
            tags
        }
    }

    pub fn serialize(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn deserialize(json: &str) -> Result<Task, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UserProgress {
    course: u32,    // Language
    unit: u32,      // Beginner/intermediate/UI/A&DS
    sector: u32,    // Syntax/loops/objects/inheritance
    level: u32,     // Loops -> for/while/do while
    task: u32       // 5-10 tasks
}

impl UserProgress {
    pub fn new() -> UserProgress {
        UserProgress {
            course: 0,
            unit: 0,
            sector: 0,
            level: 0,
            task: 0
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UserLevel {
    level: u32,
    xp: u32
}

impl UserLevel {
    pub fn new() -> UserLevel {
        UserLevel { level: 0, xp: 0 }
    }
}

#[derive(Debug, Serialize, PartialEq)]
#[serde_with::skip_serializing_none]
pub struct User<'a> {
    #[serde(with="uuid::serde::simple")]
    pub id: Uuid,
    #[serde(skip_serializing)]
    pub password_hash: &'a str,
    pub username: &'a str,
    pub email: Option<&'a str>,
    pub phone: Option<&'a str>,
    pub bio: Option<&'a str>,
    // pub avatar: ?
    #[serde(with="serde_uuid_vec")]
    pub friends: Vec<Uuid>,
    pub level: UserLevel,
    pub progress: UserProgress,
    pub auth_token: Option<Uuid> // Only if logged in
}

#[derive(Debug)]
pub enum UserError {
    UsernameExists,
    NoSuchUser,
    BadCredentials,
    BadEmail,
    BadPhone,
    MissingFields
}

impl <'a> User<'a> {

    pub fn serialize(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }


    pub fn new(username : &'a str, password_hash : &'a str, email: Option<&'a str>, phone: Option<&'a str>,  current_users : &[User]) -> Result<User<'a>, UserError> {
        if current_users.iter()
        .map(|user| user.username)
        .any(|existing_username| existing_username == username) {
            return Err(UserError::UsernameExists);
        }

        if email.is_none() && phone.is_none() {
            return Err(UserError::MissingFields);
        }

        if let Some(email) = email {
            let re = Regex::new(r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#).expect("Couldn't create regex");
            if !re.is_match(email) {
                return Err(UserError::BadEmail); 
            }
        }

        if let Some(phone) = phone {
            let re = Regex::new(r#"^(?:\+48)?[0-9]{9}$"#).expect("Couldn't create regex");
            if !re.is_match(phone) {
                return Err(UserError::BadPhone);
            }
        }

        Ok(User {
            id : Uuid::new_v4(),
            password_hash,
            username,
            email,
            phone,
            bio: None,
            // avatar: ?
            friends: Vec::new(),
            level: UserLevel::new(),
            progress: UserProgress::new(),
            auth_token: None
        })
    }

    pub fn login(username : &'a str, password_hash : &'a str, current_users : &'a [User]) -> Result<&'a User<'a>, UserError> {
        let matching_users: Vec<&User> = current_users.iter()
            .filter(|user| user.username == username)
            .collect();

        if let Some(user) = matching_users.first() {
            if user.password_hash != password_hash {
                return Err(UserError::BadCredentials);
            }

            Ok(user) 
        } else {
            Err(UserError::NoSuchUser)
        }
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
    use bcrypt::{hash, DEFAULT_COST};
    use sqlx::MySqlConnection;
    use crate::database;

    #[tokio::test]
    async fn test_task_serialization() {
        let task = Task::new("Test task".to_string(), TaskContent::Open(OpenQuestionTask { content: "Code an AGI. You have 2 minutes and cannot use google".to_string() }),
            HashSet::from([Tag{id: Uuid::new_v4(), name: "AI".to_string()}, Tag{id: Uuid::new_v4(), name: "Programming".to_string()}]));
        let serialized = task.serialize().unwrap();
        let deserialized = Task::deserialize(&serialized).unwrap();
        assert_eq!(task, deserialized);
    }

    #[test]
    fn test_user_serialization() {
        let password_hash = hash("password", DEFAULT_COST).unwrap();
        let user1 = User::new("testuser", &password_hash, Some("test@test.com"), Some("123456789"), &[]).unwrap();
        let password_hash = hash("qwerty", DEFAULT_COST).unwrap();
        let user2 = User {
            username: "testuser2",
            id: Uuid::new_v4(),
            password_hash: &password_hash,
            email: Some("test2@test.com"),
            phone: None,
            bio: Some("test bio"),
            friends: vec![user1.id],
            level: UserLevel::new(),
            progress: UserProgress::new(),
            auth_token: Some(Uuid::new_v4())
            };
        
        user2.serialize().expect("Couldn't serialize");
    }
    
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
    
    #[tokio::test]
    async fn test_task_db_insertion() {
        let binding = database::initialize_db_pool().await;
        let pool = binding.lock().await;
        let content = OpenQuestionTask { content: "Code an AGI. You have 2 minutes and cannot use google".to_string() };
        let tags = HashSet::from([Tag::new("AI".to_string(), &pool).await, Tag::new("AGI".to_string(), &pool).await]);
        let task = Task::new("Test task".to_string(), TaskContent::Open(content), tags);
        
        let result = task.insert(&pool).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_task_db_read() {
        let binding = database::initialize_db_pool().await;
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
        let binding = database::initialize_db_pool().await;
        let pool = binding.lock().await;
        let content = OpenQuestionTask { content: "Code an AGI. You have 2 minutes and cannot use google".to_string() };
        let tags = HashSet::from([Tag::new("AI".to_string(), &pool).await, Tag::new("AGI".to_string(), &pool).await]);
        let task = Task::new("Test task".to_string(), TaskContent::Open(content), tags);
        
        task.insert(&pool).await;
        
        let result = Task::delete(task.id, &pool).await;
        assert_eq!(result.is_ok(), true);
    }
}