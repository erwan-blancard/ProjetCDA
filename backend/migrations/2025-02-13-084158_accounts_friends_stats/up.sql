CREATE TABLE accounts (
  id SERIAL PRIMARY KEY,
  username VARCHAR(32) UNIQUE NOT NULL,
  email VARCHAR(255) UNIQUE NOT NULL,
  password VARCHAR(255) NOT NULL,
  premium BOOLEAN NOT NULL DEFAULT FALSE,
  suspended BOOLEAN NOT NULL DEFAULT FALSE
);


CREATE TABLE account_stats (
  id SERIAL PRIMARY KEY,
  account_id INT UNIQUE NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
  first_log TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  last_log TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  games_played BIGINT NOT NULL DEFAULT 0,
  games_won BIGINT NOT NULL DEFAULT 0,
  wallet BIGINT NOT NULL DEFAULT 0,
  experience BIGINT NOT NULL DEFAULT 0,
  level INT NOT NULL DEFAULT 0,
  season_rank INT NOT NULL DEFAULT 0,
  best_rank INT NOT NULL DEFAULT 0
);


CREATE TABLE friends (
  id SERIAL PRIMARY KEY,
  account1 INT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
  account2 INT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
  status INT NOT NULL DEFAULT 0
);

-- NOT USED

CREATE TYPE cosmetic_type AS ENUM ('other');
CREATE TABLE cosmetics (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  price INT NOT NULL DEFAULT 1,
  type cosmetic_type NOT NULL DEFAULT 'other'
);


CREATE TABLE collection_cosmetics (
  id SERIAL PRIMARY KEY,
  account_id INT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
  cosmetic_id INT NOT NULL REFERENCES cosmetics(id) ON DELETE CASCADE
);


CREATE TYPE card_element AS ENUM('fire', 'water', 'wind', 'earth');
CREATE TYPE card_type AS ENUM('weapon', 'spell', 'food');
CREATE TABLE cards (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  element card_element NOT NULL,
  type card_type NOT NULL,
  stars INT NOT NULL DEFAULT 1,
  disabled BOOL NOT NULL DEFAULT FALSE
);


CREATE TABLE collection_cards (
  id SERIAL PRIMARY KEY,
  account_id INT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
  card_id INT NOT NULL REFERENCES cards(id) ON DELETE CASCADE
);
