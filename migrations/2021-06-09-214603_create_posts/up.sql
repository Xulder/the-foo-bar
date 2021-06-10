-- Your SQL goes here
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    slug TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    author INTEGER NOT NULL REFERENCES users ON DELETE CASCADE,
    published BOOL NOT NULL DEFAULT 'f'
)

