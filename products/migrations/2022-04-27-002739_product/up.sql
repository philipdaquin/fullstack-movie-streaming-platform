-- Your SQL goes here

CREATE TABLE IF NOT EXISTS products (
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    price INT,
    weight INT
);