use crate::ctx::Ctx;
use crate::model::task::{Task, TaskBmc, TaskForCreate};
use crate::model::ModelManager;
use crate::web::Result;

use super::ParamsForCreate;

pub async fn create_task(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<TaskForCreate>,
) -> Result<Task> {
	let data = params.data;
	let id = TaskBmc::create(&ctx, &mm, data).await?;
	let task = TaskBmc::get(&ctx, &mm, id).await?;
	Ok(task)
}
