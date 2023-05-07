use std::sync::Arc;

use crate::Database;

#[derive(Clone, Debug)]
pub(crate) struct AppState {
    pub(crate) database: Database,
}

impl AppState {
    pub(crate) fn new(database: Database) -> Self {
        Self { database }
    }
}
