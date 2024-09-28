use axum::http::HeaderMap;
use axum::{http::StatusCode, response::IntoResponse};

use crate::infra::template::render;
use crate::{Error, Result};

pub struct ResponseBuilder {
    template_name: Option<String>,
    headers: HeaderMap,
}

impl Default for ResponseBuilder {
    fn default() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());

        Self {
            template_name: Option::default(),
            headers,
        }
    }
}

impl ResponseBuilder {
    pub fn with_template(self, template_name: &str) -> Self {
        Self {
            template_name: Some(template_name.to_owned()),
            ..self
        }
    }

    pub fn build(self) -> Result<Response> {
        let mut body = "".to_owned();

        if let Some(template_name) = self.template_name {
            body = render(&template_name)?
        }

        Ok(Response {
            body,
            headers: self.headers,
        })
    }
}

pub struct Response {
    body: String,
    headers: HeaderMap,
}

impl IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, self.headers, self.body).into_response()
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", self)).into_response()
    }
}
