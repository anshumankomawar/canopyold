-- Add down migration script here

ALTER TABLE BDocumentDirectories
DROP COLUMN parent_id;
