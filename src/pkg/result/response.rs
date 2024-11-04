use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

use super::status::Status;

pub struct Results<T>(pub Option<T>)
where
    T: Serialize;

impl<T> IntoResponse for Results<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let Results(data) = self;
        let status = Status::OK(data);
        Json(status.to_reply()).into_response()
    }
}

pub enum Errors {
    Error(i32, String),
    ErrBadRequest(Option<String>),
    ErrUnauthorized(Option<String>),
    ErrNotFound(Option<String>),
    ErrMethodNotAllow(Option<String>),
    ErrInternalServerError(Option<String>),
    ErrData(Option<String>),
    ErrService(Option<String>),
}

use Errors::*;

impl IntoResponse for Errors {
    fn into_response(self) -> Response {
        let status: Status<()> = match self {
            // common errors
            Error(code, msg) => Status::Err(code, msg),
            ErrBadRequest(msg) => Status::Err(400, msg.unwrap_or(String::from("Bad Request"))),
            ErrUnauthorized(msg) => Status::Err(401, msg.unwrap_or(String::from("Unauthorized"))),
            ErrNotFound(msg) => Status::Err(404, msg.unwrap_or(String::from("Not Found"))),
            ErrMethodNotAllow(msg) => {
                Status::Err(404, msg.unwrap_or(String::from("Method Not Allowed")))
            }
            ErrInternalServerError(msg) => {
                Status::Err(500, msg.unwrap_or(String::from("Internal Server Error")))
            }
            // more biz errors
            ErrData(msg) => Status::Err(1001, msg.unwrap_or(String::from("Data Error"))),
            ErrService(msg) => Status::Err(1002, msg.unwrap_or(String::from("Service Error"))),
        };
        Json(status.to_reply()).into_response()
    }
}

pub type Result<T> = std::result::Result<T, Errors>;
