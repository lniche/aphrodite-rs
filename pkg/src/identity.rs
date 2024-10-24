use anyhow::Result;
use base64::{prelude::BASE64_STANDARD, Engine};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;

use crate::{config, crypto::aes::CBC};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    i: u64,
    t: String,
}

impl Identity {
    pub fn new(id: u64, token: String) -> Self {
        Self {
            i: id,
            t: token,
        }
    }

    pub fn empty() -> Self {
        Self {
            i: 0,
            t: String::from(""),
        }
    }

    pub fn from_auth_token(token: String) -> Self {
        let cipher = match BASE64_STANDARD.decode(token) {
            Err(e) => {
                tracing::error!(error = ?e, "error invalid auth_token");
                return Identity::empty();
            }
            Ok(v) => v,
        };
        let secret = match config::global().get_string("app.secret") {
            Err(e) => {
                tracing::error!(error = ?e, "error missing config(app.secret)");
                return Identity::empty();
            }
            Ok(v) => v,
        };
        let key = secret.as_bytes();
        let plain = match CBC(key, &key[..16]).decrypt(&cipher) {
            Err(e) => {
                tracing::error!(error = ?e, "error invalid auth_token");
                return Identity::empty();
            }
            Ok(v) => v,
        };

        match serde_json::from_slice::<Identity>(&plain) {
            Err(e) => {
                tracing::error!(error = ?e, "error invalid auth_token");
                Identity::empty()
            }
            Ok(identity) => identity,
        }
    }

    pub fn to_auth_token(&self) -> Result<String> {
        let secret = config::global().get_string("app.secret")?;
        let key = secret.as_bytes();

        let plain = serde_json::to_vec(self)?;
        let cipher = CBC(key, &key[..16]).encrypt(&plain, None)?;

        Ok(BASE64_STANDARD.encode(cipher))
    }

    pub fn id(&self) -> u64 {
        self.i
    }

    pub fn match_token(&self, token: String) -> bool {
        self.t == token
    }

}

impl Display for Identity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.i == 0 {
            return write!(f, "<none>");
        }

        write!(f, "id:{}|token:{}", self.i, self.t)
    }
}
