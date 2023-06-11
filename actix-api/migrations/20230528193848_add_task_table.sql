-- Add migration script here
CREATE TABLE tasks(
    id TEXT NOT NULL PRIMARY KEY,
    user_id TEXT NOT NULL,
    description TEXT NOT NULL,
    complete INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
)

--Creates task table with user id foreign key - One user to many tasks