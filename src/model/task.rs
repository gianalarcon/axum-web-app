use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::Result;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// region:		--- Task types
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Task {
	pub id: u64,
	pub title: String,
}

#[derive(Deserialize)]
pub struct TaskForCreate {
	pub title: String,
}

#[derive(Deserialize)]
pub struct TaskForUpdate {
	pub title: Option<String>,
}
// endregion: --- Task types

// region:		--- TaskBmc
pub struct TaskBmc;

impl TaskBmc {
	pub async fn create(
		_ctx: &Ctx,
		mm: &ModelManager,
		task_c: TaskForCreate,
	) -> Result<u64> {
		let db = mm.db();

		todo!()
	}
}
