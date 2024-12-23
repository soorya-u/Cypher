use super::{
    queries::user::{FETCH_USERS_BY_EMAIL, INSERT_INTO_USERS},
    Database,
};
use crate::invokable::{ErrorAction, ErrorPayload, ErrorType};

#[derive(Debug, Default, sqlx::FromRow, serde::Serialize, serde::Deserialize)]
#[sqlx(default)]
pub struct User {
    pub full_name: String,
    pub email: String,
    pub hashed_password: String,
    pub unique_key: String,
    pub hash_salt: String,
}

impl User {
    pub fn new(
        full_name: &str,
        email: &str,
        hashed_password: &str,
        unique_key: &str,
        hash_salt: &str,
    ) -> Self {
        return User {
            full_name: full_name.to_string(),
            email: email.to_string(),
            hash_salt: hash_salt.to_string(),
            hashed_password: hashed_password.to_string(),
            unique_key: unique_key.to_string(),
        };
    }

    pub async fn insert_user_to_db(self) -> Result<(), ErrorPayload> {
        let db = Database::new().await?;

        sqlx::query(INSERT_INTO_USERS)
            .bind(&self.email)
            .bind(&self.full_name)
            .bind(&self.hashed_password)
            .bind(&self.unique_key)
            .bind(&self.hash_salt)
            .execute(&db.pool)
            .await
            .map_err(ErrorPayload::from_error_with_closure(
                ErrorType::Internal,
                "",
                ErrorAction::None,
                "Unable to execute Query",
            ))?;

        Ok(())
    }

    pub async fn from_email(email: &str) -> Result<Self, ErrorPayload> {
        let db = Database::new().await?;

        let res: User = sqlx::query_as(FETCH_USERS_BY_EMAIL)
            .bind(email)
            .fetch_one(&db.pool)
            .await
            .map_err(ErrorPayload::from_error_with_closure(
                ErrorType::Internal,
                "",
                ErrorAction::None,
                "Unable to fetch users from DB",
            ))?;

        return Ok(res);
    }
}
