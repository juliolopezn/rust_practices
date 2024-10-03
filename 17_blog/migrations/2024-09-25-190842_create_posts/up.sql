-- Your SQL goes here
CREATE TABLE posts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR NOT NULL,
    slug VARCHAR NOT NULL,
    body VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now()
)
