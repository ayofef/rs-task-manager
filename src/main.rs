mod queries;
mod route_utils;
mod routes;

#[macro_use]
extern crate dotenvy_macro;

// use dotenvy::dotenvy;
use axum::{
    Router,
    routing::{get, post},
};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

use routes::{
    create_task_route::create_task_route, delete_task_route::delete_task_route,
    index_route::index_route, list_tasks_route::list_tasks_route,
    update_task_route::update_task_route,
};

#[tokio::main]
async fn main() {
    let database_url = dotenv!("DATABASE_URL");
    let database_pool_size = dotenv!("DATABASE_POOL_SIZE").parse::<u32>().unwrap_or(5);

    if database_url.is_empty() {
        panic!("Required environment variables are not set");
    }

    let pool = PgPoolOptions::new()
        .max_connections(database_pool_size)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let router_state = Arc::new(pool);

    let app = Router::new()
        .route("/", get(index_route))
        .route("/tasks", get(list_tasks_route))
        .route("/create-task", post(create_task_route))
        .route("/update-task", post(update_task_route))
        .route("/delete-task", post(delete_task_route))
        .fallback(index_route)
        .with_state(router_state);

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
