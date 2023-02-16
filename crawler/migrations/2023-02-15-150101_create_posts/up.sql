-- Your SQL goes here
CREATE TABLE hackernews (
    id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  source TEXT NOT NULL,
  rank int NOT NULL,
  publish_date timestamp NOT NULL
)