-- Add up migration script here
DROP TABLE OnlineResource;
DROP TABLE ApiResource;


CREATE TABLE ApiResources (
    id BIGSERIAL PRIMARY KEY,
    resource_id BIGSERIAL REFERENCES Resources(id),
    method VARCHAR(512) NOT NULL,
    path VARCHAR(512) NOT NULL,
    description VARCHAR(512),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE OnlineResources (
    id BIGSERIAL PRIMARY KEY,
    resource_id BIGSERIAL REFERENCES Resources(id),
    url VARCHAR(512) NOT NULL,
    description VARCHAR(512),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
