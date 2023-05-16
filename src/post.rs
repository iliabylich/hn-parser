use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub(crate) struct Post {
    pub(crate) hn_id: i64,
    pub(crate) name: String,
}

impl Default for Post {
    fn default() -> Self {
        Self {
            hn_id: 12345,
            name: String::from("Who's hiring now"),
        }
    }
}
