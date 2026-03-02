use axum::{
    routing::{get, post},
    extract::Path,
    Json,
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Маршруты
    let app = Router::new()
        .route("/", get(handler_hello))
        .route("/hello/:name", get(handler_hello_name))
        .route("/echo", post(handler_echo));

    // Адрес
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Сервер запущен на http://{}", addr);

    // Запуск
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler_hello() -> &'static str {
    "Привет от Rust-сервера!"
}

async fn handler_hello_name(Path(name): Path<String>) -> String {
    format!("Привет, {}!", name)
}

#[derive(Deserialize, Serialize)]
struct User {
    id: u64,
    username: String,
}

// Теперь всё импортировано верно, и Handler сработает
async fn handler_echo(Json(payload): Json<User>) -> Json<User> {
    println!("Получен пользователь: {}", payload.username);
    Json(payload)
}