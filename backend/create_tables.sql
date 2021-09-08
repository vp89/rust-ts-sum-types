CREATE DATABASE animals;

\c animals

CREATE TABLE IF NOT EXISTS animals (
    animal_id SERIAL PRIMARY KEY,
    animal JSONB NOT NULL,
    animal_kind_id INT NOT NULL
);
