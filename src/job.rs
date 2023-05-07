#[derive(sqlx::FromRow, Debug)]
pub(crate) struct Job {
    pub(crate) hn_id: i64,
    pub(crate) text: String,
    pub(crate) by: String,
    pub(crate) post_hn_id: i64,
}
