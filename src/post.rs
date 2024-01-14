use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub(crate) struct Post {
    pub(crate) hn_id: i64,
    pub(crate) name: String,
}
