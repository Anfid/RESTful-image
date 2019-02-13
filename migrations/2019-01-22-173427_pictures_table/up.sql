-- Create pictures table
CREATE TABLE pictures (
    id          UUID        PRIMARY KEY NOT NULL,
    name        TEXT        NOT NULL,
    image       TEXT        NOT NULL,
    created_at  TIMESTAMP   DEFAULT CURRENT_TIMESTAMP NOT NULL
);
