-- Add up migration script here
CREATE TABLE BDocumentDirectories (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW() 
);

INSERT INTO BDocumentDirectories (name) VALUES ('root');

CREATE TABLE BDocuments (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    directory_id BIGSERIAL NOT NULL REFERENCES BDocumentDirectories(id),
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
)
