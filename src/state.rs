use std::sync::Arc;

use crate::{database::Database, mailer::Gmail, views::Views};

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) database: Database,
    pub(crate) views: Arc<Views>,
    pub(crate) gmail: Gmail,
}

impl AppState {
    pub(crate) fn new(database: Database, views: Views, gmail: Gmail) -> Self {
        Self {
            database,
            views: Arc::new(views),
            gmail,
        }
    }
}
