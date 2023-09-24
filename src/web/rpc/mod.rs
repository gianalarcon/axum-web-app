// region:		--- Modules

mod task_rpc;

use crate::web::{Error, Result};
use serde::Deserialize;
use serde_json::{from_value, json, to_value, Value};

// endregion: --- Modules

// region:		--- RPC Types

/// JSON-RPC Request Body.
#[derive(Deserialize)]
struct RpcRequest {
	id: Option<Value>,
	method: String,
	params: Option<Value>,
}

#[derive(Deserialize)]
pub struct ParamsForCreate<D> {
	data: D,
}
#[derive(Deserialize)]
pub struct ParamsForUpdate<D> {
	id: i64,
	data: D,
}
#[derive(Deserialize)]
pub struct ParamsIded {
	id: i64,
}
