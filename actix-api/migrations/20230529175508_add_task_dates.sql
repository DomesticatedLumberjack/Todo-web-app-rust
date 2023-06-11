-- Add migration script here
ALTER TABLE tasks ADD create_date TEXT NOT NULL;
ALTER TABLE tasks ADD modified_date TEXT NOT NULL;