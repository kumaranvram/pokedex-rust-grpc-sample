-- Your SQL goes here
CREATE TABLE pokemons (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  types VARCHAR NOT NULL
);