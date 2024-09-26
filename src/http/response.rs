use axum::{http::StatusCode, response::IntoResponse};

use crate::infra::template::render;
use crate::{Error, Result};

#[derive(Default)]
pub struct ResponseBuilder {
    template_name: Option<String>,
}

impl ResponseBuilder {
    pub fn with_template(self, template_name: &str) -> Self {
        Self {
            template_name: Some(template_name.to_owned()),
        }
    }

    pub fn build(self) -> Result<Response> {
        let mut body = "".to_owned();

        if let Some(template_name) = self.template_name {
            body = render(&template_name)?
        }

        Ok(Response { body })
    }
}

pub struct Response {
    body: String,
}

impl IntoResponse for Response {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::OK, self.body).into_response()
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", self)).into_response()
    }
}
