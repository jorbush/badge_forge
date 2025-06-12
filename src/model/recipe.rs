use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{Bson, DateTime as BsonDateTime};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use std::fmt;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Recipe {
    pub _id: ObjectId,
    pub user_id: ObjectId,
    pub num_likes: i32,
    pub created_at: DateTime<Utc>,
}

// Custom serialization/deserialization for flexible date handling
mod flexible_date_format {
    use super::*;

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Convert chrono DateTime to SystemTime then to BSON DateTime
        let system_time: SystemTime = (*date).into();
        let bson_dt = BsonDateTime::from_system_time(system_time);
        bson_dt.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct DateTimeVisitor;

        impl<'de> de::Visitor<'de> for DateTimeVisitor {
            type Value = DateTime<Utc>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a datetime string, timestamp, or bson datetime object")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(DateTime::parse_from_rfc3339(value)
                    .or_else(|_| DateTime::parse_from_str(value, "%Y-%m-%dT%H:%M:%S%.fZ"))
                    .or_else(|_| DateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S"))
                    .map_err(|e| E::custom(format!("Failed to parse datetime: {}", e)))?
                    .with_timezone(&Utc))
            }

            fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let bson_value = Bson::deserialize(de::value::MapAccessDeserializer::new(map))?;

                match bson_value {
                    Bson::DateTime(dt) => {
                        // Convert MongoDB DateTime to chrono DateTime
                        let millis = dt.timestamp_millis();
                        let secs = millis / 1000;
                        let nsecs = ((millis % 1000) * 1_000_000) as u32;
                        Ok(DateTime::<Utc>::from_timestamp(secs, nsecs).unwrap_or_default())
                    }

                    Bson::Document(doc) => {
                        if let Some(Bson::DateTime(dt)) = doc.get("$date") {
                            let millis = dt.timestamp_millis();
                            let secs = millis / 1000;
                            let nsecs = ((millis % 1000) * 1_000_000) as u32;
                            Ok(DateTime::<Utc>::from_timestamp(secs, nsecs).unwrap_or_default())
                        } else {
                            Err(de::Error::custom(
                                "Expected $date field in datetime document",
                            ))
                        }
                    }

                    _ => Err(de::Error::custom(format!(
                        "Unexpected BSON type for date: {:?}",
                        bson_value
                    ))),
                }
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let secs = value / 1000;
                let nsecs = ((value % 1000) * 1_000_000) as u32;
                Ok(DateTime::<Utc>::from_timestamp(secs as i64, nsecs).unwrap_or_default())
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let secs = value / 1000;
                let nsecs = ((value % 1000) * 1_000_000) as u32;
                Ok(DateTime::<Utc>::from_timestamp(secs, nsecs).unwrap_or_default())
            }
        }

        deserializer.deserialize_any(DateTimeVisitor)
    }
}

impl Serialize for Recipe {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct RecipeHelper<'a> {
            #[serde(rename = "_id")]
            pub id: &'a ObjectId,
            #[serde(rename = "userId")]
            pub user_id: &'a ObjectId,
            #[serde(rename = "numLikes")]
            pub num_likes: i32,
            #[serde(rename = "createdAt", with = "flexible_date_format")]
            pub created_at: &'a DateTime<Utc>,
        }

        RecipeHelper {
            id: &self._id,
            user_id: &self.user_id,
            num_likes: self.num_likes,
            created_at: &self.created_at,
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Recipe {
    fn deserialize<D>(deserializer: D) -> Result<Recipe, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct RecipeHelper {
            #[serde(rename = "_id")]
            pub id: ObjectId,
            #[serde(rename = "userId")]
            pub user_id: ObjectId,
            #[serde(rename = "numLikes", default)]
            pub num_likes: i32,
            #[serde(rename = "createdAt", with = "flexible_date_format")]
            pub created_at: DateTime<Utc>,
        }

        let helper = RecipeHelper::deserialize(deserializer)?;

        Ok(Recipe {
            _id: helper.id,
            user_id: helper.user_id,
            num_likes: helper.num_likes,
            created_at: helper.created_at,
        })
    }
}
