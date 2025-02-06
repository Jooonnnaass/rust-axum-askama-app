use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .nest_service("/public", ServeDir::new("public"))
        .route("/", get(index));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

async fn index() -> impl IntoResponse {
    let index_template = IndexTemplate {};

    (StatusCode::OK, Html(index_template.render().unwrap())).into_response()
}
