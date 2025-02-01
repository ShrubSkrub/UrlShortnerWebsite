-- SQLBook: Code
-- Your SQL goes here
CREATE TABLE urls (
    id SERIAL PRIMARY KEY,
    long_url TEXT NOT NULL,
    short_url TEXT NOT NULL UNIQUE,
    clicks INTEGER DEFAULT 0,
    creation_date TIMESTAMP NOT NULL DEFAULT NOW(),
    expiration_date TIMESTAMP
);