use serde::ser::SerializeSeq;
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use serde::de::{self, DeserializeSeed, Deserializer, Visitor, MapAccess};
use uuid::Uuid;
use std::fmt;
use serde_json::Value;
use regex::Regex;
use serde_with;

#[derive(Serialize, Deserialize, Debug)]
pub struct Lesson<'a> {
    #[serde(with = "uuid::serde::simple")]
    pub id: Uuid,
    pub title: &'a str, // We don't want the title to be mutable/growable, so we can store it in what is essentially a [u8]
    pub content: &'a str,
}

impl <'a> Lesson<'a> {
    pub fn new(title : &'a str, content : &'a str) -> Lesson<'a> {
        Lesson {
            id : uuid::Uuid::new_v4(),
            title,
            content
        }
    }

    pub fn serialize(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn deserialize(json: &str) -> Result<Lesson, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UserProgress {
    course: u32,    // Language
    unit: u32,      // Beginner/intermediate/UI/A&DS
    sector: u32,    // Syntax/loops/objects/inheritance
    level: u32,     // Loops -> for/while/do while
    lesson: u32     // 5-10 tasks
}

impl UserProgress {
    pub fn new() -> UserProgress {
        UserProgress {
            course: 0,
            unit: 0,
            sector: 0,
            level: 0,
            lesson: 0
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

fn serialize_users_to_ids<'a, S>(users: &Vec<&'a User<'a>>, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer,
{
    let mut seq = serializer.serialize_seq(Some(users.len()))?;
    for user in users {
        seq.serialize_element(&user.id.to_string().as_str())?;
    }
    seq.end()
}


#[derive(Debug, Serialize)]
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
    #[serde(serialize_with = "serialize_users_to_ids")]
    pub friends: Vec<&'a User<'a>>,
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

    pub fn deserialize(json: &str) -> Result<User, serde_json::Error> {
        todo!();
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

struct UserSeed<'a> {
    users: &'a[&'a User<'a>]
}


#[derive(Debug)]
pub struct Answer<'a> {
    pub lesson : &'a Lesson<'a>,
    pub user : &'a User<'a>,
    pub answer: String
}

impl <'a> Serialize for Answer<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        let mut state = serializer.serialize_struct("AnswerData", 3)?;
        state.serialize_field("lesson", self.lesson.id.to_string().as_str())?;
        state.serialize_field("user", self.user.id.to_string().as_str())?;
        state.serialize_field("answer", self.answer.as_str())?;
        state.end()   
    }
}

// Remember to use this, not serde_json::from_str()!
impl<'a> Answer<'a> {
    pub fn deserialize(json : &str, users : &'a [&'a User], lessons: &'a [&'a Lesson]) -> Result<Answer<'a>, serde_json::Error> {
        let value : Value = serde_json::from_str(json)?;
        let answer_seed = AnswerSeed { users, lessons };
        answer_seed.deserialize(value)
    }

    pub fn serialize(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

struct AnswerSeed<'a> {
    users : &'a [&'a User<'a>],
    lessons : &'a [&'a Lesson<'a>]
}

impl <'de, 'a> DeserializeSeed<'de> for AnswerSeed<'a> {
    type Value = Answer<'a>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de> {
        struct AnswerVisitor<'a> {
            lesson_id: Option<Uuid>,
            user_id: Option<Uuid>,
            answer: Option<String>,
            lessons: &'a [&'a Lesson<'a>],
            users : &'a [&'a User<'a>]
        }

        impl <'de, 'a> Visitor<'de> for AnswerVisitor<'a> {
            type Value = Answer<'a>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Answer")
            }

            fn visit_map<V>(mut self, mut map: V) -> Result<Answer<'a>, V::Error>
            where
                V: MapAccess<'de>,
            {
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "lesson" => {
                            let lesson_id = map.next_value::<Uuid>()?;
                            self.lesson_id = Some(lesson_id);
                        }
                        "user" => {
                            let user_id = map.next_value::<Uuid>()?;
                            self.user_id = Some(user_id);
                        }
                        "answer" => {
                            let answer = map.next_value::<String>()?;
                            self.answer = Some(answer);
                        }
                        _ => return Err(de::Error::unknown_field(&key, FIELDS)),
                    }
                }

                let lesson = self.lessons.iter()
                    .find(|lesson| lesson.id == self.lesson_id.unwrap())
                    .ok_or_else(|| de::Error::missing_field("lesson"))?;
                let user = self.users.iter()
                    .find(|user| user.id == self.user_id.unwrap())
                    .ok_or_else(|| de::Error::missing_field("user"))?;
                let answer = self.answer.take()
                    .ok_or_else(|| de::Error::missing_field("answer"))?;

                Ok(Answer {
                    lesson,
                    user,
                    answer
                })
            }
        }
        const FIELDS: &[&str] = &["lesson", "user", "answer"];
        deserializer.deserialize_struct("Answer", FIELDS, AnswerVisitor {
            lesson_id: None,
            user_id: None,
            answer: None,
            lessons: self.lessons,
            users: self.users,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bcrypt::{hash, DEFAULT_COST};

    #[test]
    fn test_lesson_serialization() {
        let lesson = Lesson::new("Test Lesson", "Test Content");
        let serialized = lesson.serialize().unwrap();
        let deserialized = Lesson::deserialize(&serialized).unwrap();

        assert_eq!(lesson.id, deserialized.id);
        assert_eq!(lesson.title, deserialized.title);
        assert_eq!(lesson.content, deserialized.content);
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
            friends: vec![&user1],
            level: UserLevel::new(),
            progress: UserProgress::new(),
            auth_token: Some(Uuid::new_v4())
            };
        if let Ok(serialized) = user2.serialize() {
            todo!()
        }
    }
    
    #[test]
    fn test_answer_serialization() {

    }
}