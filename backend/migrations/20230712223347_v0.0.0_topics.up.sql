-- Create the topics and documentation table

CREATE TABLE Topics (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(512) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE Resources (
    id BIGSERIAL PRIMARY KEY,
    topic_id BIGSERIAL REFERENCES Topics(id),
    description VARCHAR(512),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE ApiResource (
    id BIGSERIAL PRIMARY KEY,
    resource_id BIGSERIAL REFERENCES Resources(id),
    method VARCHAR(512) NOT NULL,
    path VARCHAR(512) NOT NULL,
    description VARCHAR(512),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE OnlineResource (
    id BIGSERIAL PRIMARY KEY,
    resource_id BIGSERIAL REFERENCES Resources(id),
    url VARCHAR(512) NOT NULL,
    description VARCHAR(512),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
-- SELECT *
-- FROM Topics t, Resources r, ApiResources ar
-- WHERE t.resource_id = r.id AND r.id = ar.resource_id;
