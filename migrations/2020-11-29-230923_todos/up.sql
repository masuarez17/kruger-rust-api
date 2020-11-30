-- Your SQL goes here
CREATE TABLE todos(
    id SERIAL PRIMARY KEY,
    todotext VARCHAR NOT NULL,
    done BOOLEAN NOT NULL,
    username VARCHAR NOT NULL
);

CREATE TABLE users(
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    email VARCHAR NOT NULL
);