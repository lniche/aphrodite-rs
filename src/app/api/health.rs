use crate::pkg::result::response::{Result, Results};

pub async fn ping() -> Result<Results<String>> {
    Ok(Results(Some("pong".to_string())))
}

pub async fn root() -> Result<Results<String>> {
    Ok(Results(Some("Thank you for using aphrodite!".to_string())))
}
