use axum::extract::State;
use sqlx::{pool::Pool, postgres::Postgres};
use std::sync::Arc;

pub type DbPoolAsState = State<Arc<Pool<Postgres>>>;
