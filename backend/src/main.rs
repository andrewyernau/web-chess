use axum::{
    routing::{get, post},
    Router,
    response::{Html, IntoResponse, Redirect},
    http::{HeaderMap},
    
};

use std::net::SocketAddr;
use tokio::net::TcpListener;
mod templates;
use templates::{
    FullIndex, FullFormRegistration, FullFormTournaments,FullBoard,
    IndexContent, FormRegistrationContent, FormTournamentsContent,Board
};

use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/form", get(form_registration_handler))
        .route("/board", get(board_handler))
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
        IndexContent {}.into_response()
    } else {
        FullIndex {}.into_response()
    }
}

async fn form_registration_handler(headers: HeaderMap) -> impl IntoResponse {
    if headers.contains_key("HX-Request") {
        FormRegistrationContent {}.into_response()
    } else {
        FullFormRegistration {}.into_response()
    }
}

async fn form_tournaments_handler(headers: HeaderMap) -> impl IntoResponse {
    if headers.contains_key("HX-Request") {
        FormTournamentsContent {}.into_response()
    } else {
        FullFormTournaments {}.into_response()
    }
}

async fn board_handler(headers: HeaderMap) -> impl IntoResponse {
    if headers.contains_key("HX-Request") {
        Board {}.into_response()
    } else {
        FullBoard {}.into_response()
    }
}

async fn handle_registration_form() -> Html<String> {
    Html(r#"
        <div class="success-toast" id="auto-toast">
            ¡Inscripción enviada con éxito!
        </div>
    "#.to_string())
}

async fn handle_tournament_form() -> Html<String> {
    Html(r#"
        <div class="success-toast" id="auto-toast">
            ¡Inscripción al torneo enviada con éxito!
        </div>
    "#.to_string())
}

async fn anything_else() -> Redirect {
    Redirect::to("/")
}