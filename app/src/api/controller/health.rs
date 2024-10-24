use pkg::result::response::{ ApiOK, Result};

pub async fn ping() -> Result<ApiOK<String>> {
    Ok(ApiOK(Some("pong".to_string())))
}

pub async fn home() -> Result<ApiOK<String>> {
    Ok(ApiOK(Some("Thank you for using aphrodite!".to_string())))
}
