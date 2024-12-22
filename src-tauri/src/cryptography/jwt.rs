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
    pub fn create_jwt(secret: &str, email: &str) -> Result<String, String> {
        let expiration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis()
            + 1000 * 60 * 60 * 24 * 15; // 1/2 month expiration

        let claims = Claims {
            sub: String::from(email),
            exp: expiration as usize,
        };

        let header = Header::default();
        let token = encode(&header, &claims, &EncodingKey::from_secret(secret.as_ref()))
            .expect("Failed to create token");

        Ok(token)
    }

    pub fn read_jwt(token: &str) -> Result<String, String> {
        let decoding_key = DecodingKey::from_secret(&[]); // Empty secret key
        let validation = Validation::default();

        let token_data =
            decode::<Claims>(token, &decoding_key, &validation).expect("Unable to read JWT");
        Ok(token_data.claims.sub)
    }

    pub fn is_jwt_expired(token: &str) -> Result<bool, String> {
        let decoding_key = DecodingKey::from_secret(&[]); // Empty secret key
        let validation = Validation::default();

        let token_data =
            decode::<Claims>(token, &decoding_key, &validation).expect("Unable to read JWT");

        if SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            < token_data.claims.exp as u128
        {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }
}
