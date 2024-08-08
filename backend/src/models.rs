pub mod user;
pub mod task;
pub mod answer;

pub mod serde_uuid_vec {
    use serde::{self, Serializer, Deserializer, Serialize, Deserialize};
    use uuid::Uuid;

    pub fn serialize<S>(vec: &[Uuid], serializer: S) -> Result<S::Ok, S::Error>
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






