use std::env::{self};

use axum::{
    response::{Html, IntoResponse}, routing::{get, get_service}, Json, Router
};
use serde::{Deserialize, Serialize};
use tokio::fs::read_to_string;
use tower_http::services::{ServeDir, ServeFile};
use once_cell::sync::Lazy;

#[derive(Serialize, Deserialize)]
pub struct WeatherForecast {
    pub date: String,
    pub temperature_c: f64,
    pub temperature_f: f64,
    pub summary: String,
    pub city : String,
}

#[derive(Serialize, Deserialize)]
pub struct Movie {
    pub title: String,
    pub director: String,
    pub year: i32,
}

impl Default for Movie {
    fn default() -> Self {
        Self {
            title: "Star Wars".to_string(),
            director: "George Lucas".to_string(),
            year: 1977,
        }
    }
}

static EXE_PATH: Lazy<String> = Lazy::new(|| {
    // 한번 만 호출
    let exe_path = env::current_exe().expect("Failed to get current executable path");
    let exe_path = exe_path.parent().expect("Failed to get parent directory").to_path_buf();
    exe_path.display().to_string()
});

async fn render_page(page: &str) -> Html<String> {
    let file_path = format!("{}/static/{}.html", *EXE_PATH, page);
    let content = match read_to_string(file_path).await {
        Ok(content) => content,
        Err(_) => "<h1>404 Not Found</h1>".to_string(),
    };
    Html(content)
}

async fn index() -> Html<String> {
    render_page("index").await
}

async fn movie() -> Html<String> {
    render_page("movie").await
}

async fn mypage() -> Html<String> {
    render_page("mypage").await
}

async fn get_movies() -> impl IntoResponse {
    let mut movies = Vec::new();
    movies.push(Movie::default());
    movies.push(Movie {
        title: "The Empire Strikes Back".to_string(),
        director: "Irvin Kershner".to_string(),
        year: 1980,
    });
    movies.push(Movie {
        title: "Return of the Jedi".to_string(),
        director: "Richard Marquand".to_string(),
        year: 1983,
    });
    Json(movies)
}

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("localhost:{}", port);

    println!("Listening on http://{}", addr);

    let asset_path = format!("{}/static/assets", *EXE_PATH);
    let static_asset_files = get_service(ServeDir::new(asset_path))
    .handle_error(|error| async move {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled error: {}", error),
        )
    });

    let static_path = format!("{}/static", *EXE_PATH);
    let vite_path = ServeFile::new(format!("{}/vite.svg", static_path));

    let app = Router::new()
        .route("/", get(index)) // 기본 경로
        .route("/movie", get(movie)) // movie.html 반환
        .route("/mypage", get(mypage)) // mypage.html 반환
        .route("/api/movie", get(get_movies))
        .nest_service("/vite.svg", vite_path)
        .nest_service("/assets", static_asset_files);
        // .fallback_service(static_files);

    let tcp_listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(tcp_listener, app).await.unwrap();
}