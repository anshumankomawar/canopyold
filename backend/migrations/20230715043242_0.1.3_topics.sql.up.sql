-- Add up migration script here
DROP TABLE ApiResources;
DROP TABLE OnlineResources;
DROP TABLE Topics;

CREATE TABLE Topics (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(1023) NOT NULL, 
    components JSON NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE Tags (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE INDEX idx_tags_name ON Tags(name);

CREATE TABLE TopicTags (
    id BIGSERIAL PRIMARY KEY,
    topic_id BIGSERIAL NOT NULL,
    tag_id BIGSERIAL NOT NULL,
    FOREIGN KEY (topic_id) REFERENCES Topics(id),
    FOREIGN KEY (tag_id) REFERENCES Tags(id),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
