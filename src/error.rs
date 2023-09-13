use rocket::Request;
use rocket::http::ContentType;
use rocket::response::{self, Responder, Response,};
use serde_json::*;
use std::io::Cursor;
use self::Error::*;

#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("UnauthenticatedError: The operation failed because the client is not authenticated.")]
    UnauthenticatedError
}

impl Error {
    fn message(&self) -> String {
        match self {
            UnauthenticatedError => format!("{}", self),
            #[cfg(debug_assertions)]
            #[allow(unreachable_patterns)]
            _ => "undefined".into(),
        }
    }
}

impl<'r> Responder<'r, 'static> for Error {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let payload = to_string(&json!({
            "status": "error",
            "message": self.message(),
        }))
        .unwrap();
        Response::build()
            .sized_body(payload.len(), Cursor::new(payload))
            .header(ContentType::new("application", "json"))
            .ok()
    }
}