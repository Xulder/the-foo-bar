-- Your SQL goes here
CREATE TABLE users
(
    id       SERIAL PRIMARY KEY,
    username VARCHAR(20) NOT NULL UNIQUE,
    email    TEXT        NOT NULL UNIQUE,
    hash     TEXT        NOT NULL
)