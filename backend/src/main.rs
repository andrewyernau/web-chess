use axum::{
    routing::{get, post},
    Router,
    response::{Html, IntoResponse, Redirect},
    http::{HeaderMap, StatusCode},
    extract::State,
};

use std::net::SocketAddr;
use tokio::net::TcpListener;
mod templates;
use templates::{Index, FormRegistration, FormTournaments};
use askama::Template;

use tower_http::services::ServeDir;
use std::sync::Arc;



#[derive(Clone)]
struct AppState {
}

#[tokio::main]
async fn main() {
    let app_state = AppState {};

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/form", get(form_registration_handler))
        .route("/form-tournaments", get(form_tournaments_handler))
        .route("/form", post(handle_registration_form))
        .route("/register-tournament", post(handle_tournament_form))
        .nest_service("/static", ServeDir::new("backend/static"))
        .fallback(anything_else);

    let addr = SocketAddr::from(([127, 0, 0, 1], 11080));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Listening on http://{}", addr);
    axum::serve(listener, app.into_make_service()).await.unwrap();
}


async fn index_handler(headers: HeaderMap) -> impl IntoResponse {
    if headers.contains_key("HX-Request") {
        Html(Index {}.render().unwrap()).into_response()
    } else {
        Html(Index {}.render().unwrap()).into_response()
    }
}

async fn form_registration_handler(headers: HeaderMap) -> impl IntoResponse {
    if headers.contains_key("HX-Request") {
        Html(FormRegistration {}.render().unwrap()).into_response()
    } else {
        Html(FormRegistration {}.render().unwrap()).into_response()
    }
}

async fn form_tournaments_handler(headers: HeaderMap) -> impl IntoResponse {
    if headers.contains_key("HX-Request") {
        Html(FormTournaments {}.render().unwrap()).into_response()
    } else {
        Html(FormTournaments {}.render().unwrap()).into_response()
    }
}

async fn handle_registration_form(
) -> Html<String> {
    Html("<div class=\"success-toast\">¡Inscripción enviada con éxito!</div>".to_string())
}
async fn handle_tournament_form(
) -> Html<String> {
    Html("<div class=\"success-toast\">¡Inscripción al torneo enviada con éxito!</div>".to_string())
}

async fn anything_else() -> Redirect {
    Redirect::to("/")
}
