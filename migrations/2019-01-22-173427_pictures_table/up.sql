-- Create pictures table
CREATE TABLE pictures (
    name        TEXT        PRIMARY KEY,
    created_at  TIMESTAMP   DEFAULT CURRENT_TIMESTAMP NOT NULL,
    filepath    TEXT,
    description TEXT
);
