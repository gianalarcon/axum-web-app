/*!
Model Layer

Design:

- The Model Layer normalizes the application's data type structures and access.
- All application code data access must go through the Model Layer.
- The `ModelManager` holds the internal states/resources needed by ModelController to access data.
  (e.g., db_pool, S3 client, redis client)
- Model Controllers (e.g., `TaskBmc`, `ProjectBmc`) implement CRUD and other data access methods on a given "entity"
  (e.g.m `Task`, `Project`).
  (`Bmc` is short for Backend Model Controller).
- In frameworks like Axum, Tauri, `ModelManager` are tipically used as App State.
- ModelManager are designed to be passed as an argument to all Model Controllers functions.

*/

// region:    --- Modules

mod error;
mod store;
pub mod task;
pub use self::error::{Error, Result};
use self::store::{new_db_pool, Db};

// endregion: --- Modules

#[derive(Clone)]
pub struct ModelManager {
	db: Db,
}

impl ModelManager {
	pub async fn new() -> Result<Self> {
		let db = new_db_pool().await?;
		Ok(ModelManager { db })
	}

	// Returns the sqlx db pool reference.
	// Only for model layer

	pub(in crate::model) fn db(&self) -> &Db {
		&self.db
	}
}
