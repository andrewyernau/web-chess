use axum::{response::{Html, IntoResponse}};
use askama::Template;
use axum::http::StatusCode;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {}

#[derive(Template)]
#[template(path = "form-registration.html")]
pub struct FormRegistration {}
impl IntoResponse for FormRegistration {
    fn into_response(self) -> axum::response::Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Template error: {}", e)
            ).into_response(),
        }
    }
}

#[derive(Template)]
#[template(path = "form-tournaments.html")]
pub struct FormTournaments {}
impl IntoResponse for FormTournaments {
    fn into_response(self) -> axum::response::Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Template error: {}", e)
            ).into_response(),
        }
    }
}