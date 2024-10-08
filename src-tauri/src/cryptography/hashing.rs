use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub struct Hashing {
    salt: SaltString,
}

impl Hashing {
    pub fn new() -> Self {
        let salt = SaltString::generate(&mut OsRng);
        Hashing { salt }
    }

    pub fn from(salt: &str) -> Self {
        let salt = SaltString::from_b64(salt).unwrap();
        Hashing { salt }
    }

    pub fn get_salt(&self) -> String {
        String::from(self.salt.as_str())
    }

    pub fn hash_data(&self, data: &str) -> String {
        let argon2 = Argon2::default();
        argon2
            .hash_password(data.as_bytes(), &self.salt)
            .unwrap()
            .to_string()
    }

    pub fn verify_hash(&self, data: &str, hash: &str) -> bool {
        let parsed_hash = PasswordHash::new(hash).unwrap();
        Argon2::default()
            .verify_password(data.as_bytes(), &parsed_hash)
            .is_ok()
    }
}
