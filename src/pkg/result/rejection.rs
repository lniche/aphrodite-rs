use axum::{
    extract::rejection::JsonRejection,
    response::{IntoResponse, Response},
};
use axum_extra::extract::WithRejection;
use thiserror::Error;

use super::response::Errors;

#[derive(Debug, Error)]
pub enum MyRejection {
    // The `#[from]` attribute generates `From<JsonRejection> for MyRejection`
    // implementation. See `thiserror` docs for more information
    #[error(transparent)]
    JSONExtractor(#[from] JsonRejection),
}

// We implement `IntoResponse` so MyRejection can be used as a response
impl IntoResponse for MyRejection {
    fn into_response(self) -> Response {
        let err = match self {
            MyRejection::JSONExtractor(x) => match x {
                JsonRejection::JsonDataError(e) => Errors::ErrData(Some(e.body_text())),
                JsonRejection::JsonSyntaxError(e) => Errors::ErrData(Some(e.body_text())),
                JsonRejection::MissingJsonContentType(e) => Errors::ErrData(Some(e.body_text())),
                _ => Errors::ErrInternalServerError(None),
            },
        };
        err.into_response()
    }
}

pub type IRejection<T> = WithRejection<T, MyRejection>;
