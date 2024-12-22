use super::{
    queries::user::{FETCH_USERS_BY_EMAIL, INSERT_INTO_USERS},
    Database,
};

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
            full_name: String::from(full_name),
            email: String::from(full_name),
            hash_salt: String::from(full_name),
            hashed_password: String::from(full_name),
            unique_key: String::from(full_name),
        };
    }

    pub async fn insert_user_to_db(self) -> Result<(), String> {
        let db = Database::new().await?;

        sqlx::query(INSERT_INTO_USERS)
            .bind(&self.email)
            .bind(&self.full_name)
            .bind(&self.hashed_password)
            .bind(&self.unique_key)
            .bind(&self.hash_salt)
            .execute(&db.pool)
            .await
            .expect("Unable to execute Query");

        Ok(())
    }

    pub async fn from_email(email: &str) -> Result<Self, String> {
        let db = Database::new().await?;

        let res: User = sqlx::query_as(FETCH_USERS_BY_EMAIL)
            .bind(email)
            .fetch_one(&db.pool)
            .await
            .expect("Unable to fetch users from DB");

        return Ok(res);
    }
}
