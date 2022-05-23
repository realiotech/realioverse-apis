-- Your SQL goes here

CREATE TABLE ethbalances (
  id SERIAL PRIMARY KEY,
  account VARCHAR NOT NULL,
  balance NUMERIC NOT NULL,
  holder BOOLEAN NOT NUll,
  last_updated timestamp default current_timestamp NOT NULL
)