-- Add migration script here

CREATE EXTENSION pgcrypto;

CREATE TABLE IF NOT EXISTS valid_roles (
    roles VARCHAR(64) PRIMARY KEY
);
INSERT INTO valid_roles (roles) VALUES 
    ('ADMIN'),
    ('CUSTOMER'),
    ('OPERATOR'),
    ('USER');

CREATE TABLE IF NOT EXISTS users (
    id uuid DEFAULT gen_random_uuid() PRIMARY KEY NOT NULL,
    email VARCHAR NOT NULL, 
    hash TEXT NOT NULL, 
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NULL,
    username VARCHAR(50) NOT NULL,
    first_name VARCHAR NOT NULL,
    last_name VARCHAR NOT NULL,
    image_url VARCHAR,
    last_login_at TIMESTAMP,
    role VARCHAR(64) REFERENCES valid_roles (roles) ON UPDATE CASCADE DEFAULT 'USER' NOT NULL

);   

CREATE INDEX user_id on users (id);

CREATE TABLE IF NOT EXISTS profiles (
    profile_id uuid DEFAULT gen_random_uuid() PRIMARY KEY NOT NULL,
    id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    username VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NULL
);