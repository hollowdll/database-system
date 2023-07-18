// Data storage API

use crate::{
    DatabaseManager,
    Logger,
};

/// Data storage API.
/// 
/// Provides methods to do database operations.
/// 
/// Logs errors and events before forwarding results to clients.
pub struct StorageApi<'a> {
    db_manager: DatabaseManager<'a>,
    logger: &'a Logger<'a>,
}

impl<'a> StorageApi<'a> {
    /// Builds storage API.
    pub fn build(
        db_manager: DatabaseManager<'a>,
        logger: &'a Logger<'a>,
    ) -> StorageApi<'a>
    {
        StorageApi {
            db_manager,
            logger,
        }
    }
}

impl<'a> StorageApi<'a> {
    // TODO
    // add methods here
}
