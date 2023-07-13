-- Add up migration script here
DROP TABLE OnlineResources;
DROP TABLE ApiResources;
DROP TABLE Resources;

CREATE TABLE ApiResources (
    id BIGSERIAL PRIMARY KEY,
    topic_id BIGSERIAL REFERENCES Topics(id),
    method VARCHAR(512) NOT NULL,
    path VARCHAR(512) NOT NULL,
    description VARCHAR(512),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE OnlineResources (
    id BIGSERIAL PRIMARY KEY,
    topic_id BIGSERIAL REFERENCES Topics(id),
    url VARCHAR(512) NOT NULL,
    description VARCHAR(512),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
