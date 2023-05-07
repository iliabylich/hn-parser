#[derive(sqlx::FromRow, Debug)]
pub(crate) struct Post {
    pub(crate) hn_id: i64,
    pub(crate) name: String,
}
