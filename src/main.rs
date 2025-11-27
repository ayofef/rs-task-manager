mod route_utils;
mod routes;

use axum::{
    Router,
    routing::{get, post},
};

use routes::{
    create_task_route::create_task_route, delete_task_route::delete_task_route,
    index_route::index_route, list_tasks_route::list_tasks_route,
    update_task_route::update_task_route,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index_route))
        .route("/tasks", get(list_tasks_route))
        .route("/create-task", post(create_task_route))
        .route("/update-task", post(update_task_route))
        .route("/delete-task", post(delete_task_route))
        .fallback(index_route);

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
