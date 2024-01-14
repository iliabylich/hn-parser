use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub(crate) struct Post {
    pub(crate) hn_id: u32,
    pub(crate) name: String,
}
