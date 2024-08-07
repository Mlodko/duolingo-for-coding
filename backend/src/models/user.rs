use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use uuid::Uuid;
use regex::Regex;
use sqlx::query;
use chrono::{prelude::*, TimeDelta};
use super::serde_uuid_vec;

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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde_with::skip_serializing_none]
pub struct User {
    #[serde(with="uuid::serde::simple")]
    pub id: Uuid,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub username: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub bio: Option<String>,
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
    MissingFields,
    DatabaseError(sqlx::Error)
}

#[derive(Debug, PartialEq)]
pub enum AuthorizationError {
    NoTokenInUser,
    TokenNotInDatabse,
    TokenExpired
}

impl User {

    pub fn serialize(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn deserialize(json: &str) -> Result<User, serde_json::Error> {
        serde_json::from_str(json)
    }
    
    pub async fn new(username : String, password_hash : String, email: Option<String>, phone: Option<String>,  pool: &MySqlPool) -> Result<User, UserError> {
        match query!(
            "SELECT id FROM users WHERE username = ?",
            username
        ).fetch_optional(pool).await {
            Ok(result) => {
                if result.is_some() {
                    return Err(UserError::UsernameExists);
                }
            },
            Err(e) => return Err(UserError::DatabaseError(e))
        }

        if email.is_none() && phone.is_none() {
            return Err(UserError::MissingFields);
        }

        if let Some(email) = &email {
            let re = Regex::new(r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#).expect("Couldn't create regex");
            if !re.is_match(email.as_str()) && email.len() > 128 {
                return Err(UserError::BadEmail); 
            }
        }

        if let Some(phone) = &phone {
            let re = Regex::new(r#"^(?:\+48)?[0-9]{9}$"#).expect("Couldn't create regex");
            if !re.is_match(phone.as_str()) && phone.len() > 32 {
                return Err(UserError::BadPhone);
            }
        }

        Ok(User {
            id : Uuid::new_v4(),
            password_hash: password_hash.to_string(),
            username: username.to_string(),
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

    pub async fn login(username : String, password_hash : String, pool: &MySqlPool) -> Result<User, UserError> {
        let user_id: Uuid = match query!(
            "SELECT id FROM users WHERE username = ? AND password_hash = ?",
            username,
            password_hash
        ).fetch_optional(pool).await {
            Ok(result) => {
                match result {
                    Some(row) => Uuid::parse_str(row.id.as_str()).expect("Couldn't parse string into Uid"),
                    None => return Err(UserError::BadCredentials)
                }
            },
            Err(e) => return Err(UserError::DatabaseError(e))
        };
        
        let mut user = match User::read(user_id, pool).await {
            Ok(user) => user,
            Err(e) => return Err(UserError::DatabaseError(e))
        };
        
        user.auth_token = Some(Uuid::new_v4());
        let mut update_transaction = pool.begin().await.map_err(UserError::DatabaseError)?;
        
        query!(
            "INSERT INTO sessions (user_id, auth_token, expiration_time, creation_time) VALUES (?, ?, ?, ?)",
            &user.id.to_string(),
            &user.auth_token.unwrap().to_string(),
            Utc::now() + chrono::Days::new(14),
            Utc::now()
        ).execute(&mut *update_transaction).await.map_err(UserError::DatabaseError)?;
        
        update_transaction.commit().await.map_err(UserError::DatabaseError)?;
        
        Ok(user)
    }
    
    pub async fn read(id: Uuid, pool: &MySqlPool) -> Result<User, sqlx::Error> {
        let mut transaction = pool.begin().await?;
        let user_row = query!("SELECT * FROM users WHERE id = ?", id.to_string())
            .fetch_one(&mut *transaction)
            .await?;
        let friends_rows = query!("SELECT user_id_2 FROM friends WHERE user_id_1 = ?", id.to_string())
            .fetch_all(&mut *transaction)
            .await?;
        let progress_row = query!("SELECT * FROM user_progress WHERE user_id = ?", id.to_string())
            .fetch_one(&mut *transaction)
            .await?;
        transaction.commit().await?;
        
        let friends : Vec<Uuid> = friends_rows.iter()
            .map(|row| Uuid::parse_str(&row.user_id_2).expect("Couldn't parse string to Uuid"))
            .collect();
        let user_progress = UserProgress {
            course: progress_row.course as u32,
            unit: progress_row.unit as u32,
            sector: progress_row.sector as u32,
            level: progress_row.level as u32,
            task: progress_row.task as u32,
        };
        
        Ok(User {
            id: Uuid::parse_str(&user_row.id).expect("Couldn't parse string to Uuid"),
            password_hash: user_row.password_hash.to_string(),
            username: user_row.username
                .to_string(),
            email: user_row.email,
            phone: user_row.phone,
            bio: user_row.bio,
            friends,
            level: UserLevel {level: user_row.level as u32, xp: user_row.xp as u32},
            progress: user_progress,
            auth_token: None
        })
    }
    
    pub async fn insert(&self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
        let mut transaction = pool.begin().await?;
        
        query!("INSERT INTO users (id, password_hash, username, email, phone, bio, level, xp) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            self.id.to_string(),
            self.password_hash,
            self.username,
            self.email,
            self.phone,
            self.bio,
            self.level.level,
            self.level.xp).execute(&mut *transaction).await?;
        
        query!("INSERT INTO user_progress (user_id, course, unit, sector, level, task) VALUES (?, ?, ?, ?, ?, ?)",
            self.id.to_string(),
            self.progress.course,
            self.progress.unit,
            self.progress.sector,
            self.progress.level,
            self.progress.task).execute(&mut *transaction).await?;
        
        for friend_id in &self.friends {
            query!("INSERT INTO friends (user_id_1, user_id_2) VALUES (?, ?)", self.id.to_string(), friend_id.to_string()).execute(&mut *transaction).await?;
        }
        
        transaction.commit().await?;
        
        Ok(())
    }
    
    pub async fn update(&self, pool: &MySqlPool) -> Result<(), sqlx::Error> {
        let mut transaction = pool.begin().await?;
        
        query!("UPDATE users
            SET password_hash = ?,
            username = ?,
            email = ?,
            phone = ?,
            bio = ?,
            level = ?,
            xp = ?
            WHERE id = ?",
            self.password_hash,
            self.username,
            self.email,
            self.phone,
            self.bio,
            self.level.level,
            self.level.xp,
            self.id.to_string()).execute(&mut *transaction).await?;
        
        query!("UPDATE user_progress
            SET course = ?,
            unit = ?,
            sector = ?,
            level = ?,
            task = ?
            WHERE user_id = ?",
            self.progress.course,
            self.progress.unit,
            self.progress.sector,
            self.progress.level,
            self.progress.task,
            self.id.to_string()).execute(&mut *transaction).await?;
        
        // Get user's friends
        let database_friend_ids = query!("SELECT user_id_2 FROM friends WHERE user_id_1 = ?", self.id.to_string()).fetch_all(&mut *transaction).await?.iter()
            .map(|record| Uuid::parse_str(record.user_id_2.to_string().as_str()).expect("Couldn't parse string to Uuid"))
            .collect::<Vec<Uuid>>();
        
        for friend_id in &self.friends {
            if !database_friend_ids.contains(friend_id) { // STRUCT YES, DB NO
                query!("INSERT INTO friends (user_id_1, user_id_2) VALUES (?, ?)", self.id.to_string(), friend_id.to_string()).execute(&mut *transaction).await?;
            }
        }
        
        for db_friend_id in database_friend_ids {
            if !self.friends.contains(&db_friend_id) { // DB YES, STRUCT NO
                query!("DELETE FROM friends WHERE user_id_1 = ? AND user_id_2 = ?", self.id.to_string(), db_friend_id.to_string()).execute(&mut *transaction).await?;
            }
        }
        
        transaction.commit().await?;
        Ok(())
    }
    
    pub async fn check_authorization(auth_token: &Option<Uuid>, pool: &MySqlPool) -> Result<Result<(), AuthorizationError>, sqlx::Error> {
        use AuthorizationError::*;
        /*
        Err(_) ->       database error
        Ok(Err(_)) ->   db ok, authorization failed
        Ok(Ok(_)) ->    db ok, authorization succeeded
        */
        match auth_token {
            None => Ok(Err(NoTokenInUser)),
            Some(token) => {
                match query!("SELECT expiration_time FROM sessions WHERE auth_token = ?", token.to_string()).fetch_optional(pool).await? {
                    None => Ok(Err(TokenNotInDatabse)),
                    Some(record) => {
                        if record.expiration_time.and_utc() < Utc::now() {
                            Ok(Err(TokenExpired))
                        } else {
                            Ok(Ok(()))
                        }
                    }
                }
            }
        }
    }
}
