-- Create pictures table
CREATE TABLE pictures (
    id          BIGSERIAL   PRIMARY KEY,
    name        TEXT        NOT NULL,
    image       TEXT        NOT NULL,
    created_at  TIMESTAMP   DEFAULT CURRENT_TIMESTAMP NOT NULL,
    description TEXT
);
