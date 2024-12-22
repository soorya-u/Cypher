pub const INSERT_INTO_USERS: &str = r#"
INSERT INTO users (email,  full_name,  hashed_password,  unique_key, hash_salt) VALUES ( $1, $2, $3, $4, $5 );
"#;

pub const FETCH_USERS_BY_EMAIL: &str = r#"
SELECT * FROM users WHERE email=$1;
"#;
