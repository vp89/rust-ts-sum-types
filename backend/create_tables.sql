CREATE DATABASE content_catalog;

\c content_catalog

CREATE TABLE IF NOT EXISTS content_entries (
    entry_id SERIAL PRIMARY KEY,
    content_url VARCHAR(500) NOT NULL,
    content_type_id INT NOT NULL,
    content_metadata JSONB NOT NULL
);
