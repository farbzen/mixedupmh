use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

pub const ERROR_RENDER: &str = "ERROR: Could not render the template!";

#[derive(Debug, displaydoc::Display, thiserror::Error)]
enum AppError {
    /// could not render template
    Render(#[from] askama::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Debug, Template)]
        #[template(path = "pages/error.html")]
        struct Tmpl {}

        let status = match &self {
            AppError::Render(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let tmpl = Tmpl {};
        if let Ok(body) = tmpl.render() {
            (status, Html(body)).into_response()
        } else {
            (status, "Something went wrong").into_response()
        }
    }
}
