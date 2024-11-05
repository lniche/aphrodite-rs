use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct Resp {
    #[schema(example = "0")]
    pub code: i32,
    #[schema(example = "ok")]
    pub message: String,
}
#[derive(Serialize)]
pub struct Reply<T>
where
    T: Serialize,
{
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

pub enum Status<T>
where
    T: Serialize,
{
    OK(Option<T>),
    Err(i32, String),
}

impl<T> Status<T>
where
    T: Serialize,
{
    pub fn to_reply(self) -> Reply<T> {
        let mut resp = Reply {
            code: 0,
            message: String::from("ok"),
            data: None,
        };
        match self {
            Status::OK(data) => {
                resp.data = data;
            }
            Status::Err(code, message) => {
                resp.code = code;
                resp.message = message;
            }
        }
        resp
    }
}
