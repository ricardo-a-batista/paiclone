use crate::http::{Response, ResponseBuilder};

use crate::Result;

pub async fn index() -> Result<Response> {
    ResponseBuilder::default()
        .with_template("home/index.html")
        .build()
}
