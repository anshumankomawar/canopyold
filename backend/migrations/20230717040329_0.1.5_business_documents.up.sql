-- Add up migration script here

ALTER TABLE BDocumentDirectories
ADD COLUMN parent_id BIGSERIAL REFERENCES BDocumentDirectories(id) NOT NULL;
