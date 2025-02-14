pub const CREATE_SCHEMA_QUERY: &str = r#"
  
  PRAGMA foreign_keys = ON;
  
  CREATE TABLE IF NOT EXISTS users (
    email              TEXT      PRIMARY KEY,
    full_name          TEXT      NOT NULL,
    hashed_password    TEXT      NOT NULL,
    unique_key         TEXT      NOT NULL UNIQUE,
    hash_salt          TEXT      NOT NULL UNIQUE,
    created_at         DATETIME  DEFAULT (datetime('now')),
    updated_at         DATETIME  DEFAULT (datetime('now')),
    is_remotely_synced BOOLEAN   DEFAULT 0 NOT NULL
  );

  CREATE TABLE IF NOT EXISTS accounts (
    id                 TEXT     PRIMARY KEY,
    platform_name      TEXT     NOT NULL,
    platform_url       TEXT     NOT NULL,
    email              TEXT,
    phone_number       TEXT,
    encypted_password  TEXT     NOT NULL,
    nonce              TEXT     NOT NULL,
    is_remotely_synced BOOLEAN  NOT NULL DEFAULT 0,
    unique_key         TEXT,
    FOREIGN KEY (unique_key) REFERENCES users(unique_key)
  );

  CREATE TABLE IF NOT EXISTS recovery_phases (
    account_id TEXT,
    phase TEXT,
    PRIMARY KEY(account_id, phase)
    FOREIGN KEY (account_id) REFERENCES accounts(id)
  );

  CREATE TABLE IF NOT EXISTS extra_details (
    field      TEXT NOT NULL,
    value      TEXT NOT NULL,
    datetype   TEXT DEFAULT ('string'),
    account_id TEXT,
    FOREIGN KEY (account_id) REFERENCES accounts(id)
  );
"#;
