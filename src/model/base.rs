use crate::ctx::Ctx;
use crate::model::task::Task;
use crate::model::ModelManager;
use crate::model::{Error, Result};
use sqlx::postgres::PgRow;
use sqlx::FromRow;

pub trait DbBmc {
	const TABLE: &'static str;
}

pub async fn get<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
where
	MC: DbBmc,
	E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
	let db = mm.db();
	let sql_statement = format!("SELECT * FROM {} WHERE id = $1", MC::TABLE);
	let err = Error::EntityNotFound {
		entity: MC::TABLE,
		id,
	};
	let entity: E = sqlx::query_as(&sql_statement)
		.bind(id)
		.fetch_optional(db)
		.await?
		.ok_or(err)?;
	Ok(entity)
}
