use crate::invokable::{ErrorAction, ErrorPayload, ErrorType};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub struct JWT {}

impl JWT {
    fn get_secret() -> Result<String, ErrorPayload> {
        let secret = std::env::var("JWT_SECRET").map_err(ErrorPayload::from_error_with_closure(
            ErrorType::Internal,
            "",
            ErrorAction::None,
            "Unable to get Environment variable",
        ))?;

        Ok(secret)
    }

    pub fn create_jwt(email: &str) -> Result<String, ErrorPayload> {
        let expiration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(ErrorPayload::from_error_with_closure(
                ErrorType::Internal,
                "",
                ErrorAction::None,
                "Time went backwards",
            ))?
            .as_millis()
            + 1000 * 60 * 60 * 24 * 15; // 15 days expiration

        let claims = Claims {
            sub: String::from(email),
            exp: expiration as usize,
        };

        let secret: String = JWT::get_secret()?;

        let header = Header::default();
        let token = encode(
            &header,
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .map_err(ErrorPayload::from_error_with_closure(
            ErrorType::Internal,
            "",
            ErrorAction::None,
            "Failed to create token",
        ))?;

        Ok(token)
    }

    pub fn decode_jwt(token: &str) -> Result<String, ErrorPayload> {
        let secret: String = JWT::get_secret()?;

        let decoding_key = DecodingKey::from_secret(secret.as_bytes());
        let validation = Validation::default();

        let token_data = decode::<Claims>(token, &decoding_key, &validation).map_err(
            ErrorPayload::from_error_with_closure(
                ErrorType::User,
                "Unable to verify your Session! Please Login again.",
                ErrorAction::Redirect,
                "Unable to read JWT",
            ),
        )?;

        if SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            > token_data.claims.exp as u128
        {
            return Err(ErrorPayload {
                details: "Jwt has been Expired".to_string(),
                error: "".to_string(),
                error_type: ErrorType::Expected,
                action_type: ErrorAction::Redirect,
                message: "Your session has been expired! Please Login Again".to_string(),
            });
        }
        Ok(token_data.claims.sub)
    }
}
