use crate::{database::Database, mailer::Gmail};

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) database: Database,
    pub(crate) gmail: Gmail,
}

impl AppState {
    pub(crate) fn new(database: Database, gmail: Gmail) -> Self {
        Self { database, gmail }
    }
}
