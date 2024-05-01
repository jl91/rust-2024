mod model;
mod response;
mod handler;

use axum::{
    response::IntoResponse,
    Json,
    routing::get,
    Router,
};
use axum::http::{HeaderValue, Method};
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::routing::post;
use tower_http::cors::CorsLayer;
use crate::handler::{
    create_todo_handler,
    delete_todo_handler,
    find_one_by_id_handler,
    todos_list_handler,
    update_todo_handler,
};

async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "Build Simple CRUD API in Rust using Axum";

    let json_reponse = serde_json::json!({
       "status": "success",
        "message": MESSAGE,
    });

    Json(json_reponse)
}

pub fn create_router() -> Router {
    let db = model::todo_db();

    Router::new()
        .route(
            "/api/healthchecker",
            get(health_checker_handler),
        )
        .route(
            "/api/todos",
            post(create_todo_handler)
                .get(todos_list_handler),
        )
        .route(
            "/api/todos/:id",
            get(find_one_by_id_handler)
                .patch(update_todo_handler)
                .delete(delete_todo_handler),
        )
        .with_state(db)
}


#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    println!("🚀 Server started successfully");

    let app = create_router().layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
