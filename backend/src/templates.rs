use axum::{response::{Html, IntoResponse}};
use askama::Template;
use axum::http::StatusCode;

#[derive(Template)]
#[template(path = "index.html")]
pub struct FullIndex {}

#[derive(Template)]
#[template(path = "form-registration.html")]
pub struct FullFormRegistration {}

#[derive(Template)]
#[template(path = "form-tournaments.html")]
pub struct FullFormTournaments {}


#[derive(Template)]
#[template(path = "index-content.html")]
pub struct IndexContent {}

#[derive(Template)]
#[template(path = "form-registration-content.html")]
pub struct FormRegistrationContent {}

#[derive(Template)]
#[template(path = "form-tournaments-content.html")]
pub struct FormTournamentsContent {}

impl IntoResponse for FullIndex {
    fn into_response(self) -> axum::response::Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Template error: {}", e)).into_response(),
        }
    }
}
impl IntoResponse for FullFormRegistration {
    fn into_response(self) -> axum::response::Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Template error: {}", e)).into_response(),
        }
    }
}
impl IntoResponse for FullFormTournaments {
    fn into_response(self) -> axum::response::Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Template error: {}", e)).into_response(),
        }
    }
}
impl IntoResponse for IndexContent {
    fn into_response(self) -> axum::response::Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Template error: {}", e)).into_response(),
        }
    }
}
impl IntoResponse for FormRegistrationContent {
    fn into_response(self) -> axum::response::Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Template error: {}", e)).into_response(),
        }
    }
}
impl IntoResponse for FormTournamentsContent {
    fn into_response(self) -> axum::response::Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Template error: {}", e)).into_response(),
        }
    }
}