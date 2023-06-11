-- Add migration script here
CREATE TABLE users(
    id TEXT PRIMARY KEY NOT NULL,
    username TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    create_time TEXT NOT NULL
)