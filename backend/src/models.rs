use serde::{ser::SerializeStruct, Deserialize, Serialize};
use serde::de::{self, DeserializeSeed, Deserializer, Visitor, MapAccess};
use uuid::Uuid;
use std::fmt;
use serde_json::Value;

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

#[derive(Serialize, Deserialize, Debug)]
pub struct User<'a> {
    #[serde(with = "uuid::serde::simple")]
    pub id: Uuid,
    pub password_hash: &'a str,
    pub username: &'a str,
    pub progress: i32,
}

#[derive(Debug)]
pub enum UserError {
    UsernameExists,
    NoSuchUser,
    BadCredentials
}

impl <'a> User<'a> {

    pub fn serialize(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn deserialize(json: &str) -> Result<User, serde_json::Error> {
        serde_json::from_str(json)
    }

    pub fn new(username : &'a str, password_hash : &'a str, progress : i32, current_users : &[User]) -> Result<User<'a>, UserError> {
        if current_users.iter()
        .map(|user| user.username)
        .any(|existing_username| existing_username == username) {
            return Err(UserError::UsernameExists);
        }
        Ok(User {
            id : Uuid::new_v4(),
            password_hash,
            username,
            progress
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

#[derive(Debug)]
pub struct Answer<'a> {
    pub lesson : &'a Lesson<'a>,
    pub user : &'a User<'a>,
    pub answer: String
}

impl <'a> Serialize for Answer<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
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
        let user = User::new("testuser", &password_hash, 0, &[]).unwrap();
        let serialized = user.serialize().unwrap();
        let deserialized = User::deserialize(&serialized).unwrap();

        assert_eq!(user.id, deserialized.id);
        assert_eq!(user.username, deserialized.username);
        assert_eq!(user.progress, deserialized.progress);
    }

    #[test]
    fn test_answer_serialization() {
        let password_hash = hash("password", DEFAULT_COST).unwrap();
        let user = User::new("testuser", &password_hash, 0, &[]).unwrap();
        let lesson = Lesson::new("Test Lesson", "Test Content");
        let answer = Answer {
            lesson: &lesson,
            user: &user,
            answer: "Test Answer".to_owned(),
        };

        let serialized = answer.serialize().unwrap();
        let users = [&user];
        let lessons = [&lesson];
        let deserialized = Answer::deserialize(&serialized, &users, &lessons).unwrap();

        assert_eq!(answer.lesson.id, deserialized.lesson.id);
        assert_eq!(answer.user.id, deserialized.user.id);
        assert_eq!(answer.answer, deserialized.answer);
    }
}