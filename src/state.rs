use std::sync::Arc;

use crate::{database::Database, views::Views};

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) database: Database,
    pub(crate) views: Arc<Views>,
}

impl AppState {
    pub(crate) fn new(database: Database, views: Views) -> Self {
        Self {
            database,
            views: Arc::new(views),
        }
    }
}
