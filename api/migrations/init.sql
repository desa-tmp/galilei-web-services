CREATE EXTENSION pgcrypto;

CREATE TABLE users (
  id UUID NOT NULL DEFAULT gen_random_uuid(),
  name TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE sessions (
  id UUID NOT NULL DEFAULT gen_random_uuid(),
  expires TIMESTAMP NOT NULL,
  user_id UUID NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE galaxies (
  id UUID NOT NULL DEFAULT gen_random_uuid(),
  name TEXT NOT NULL,
  user_id UUID NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (user_id) REFERENCES users(id),
  UNIQUE (name, user_id)
);

CREATE TABLE stars (
  id UUID NOT NULL DEFAULT gen_random_uuid(),
  name TEXT NOT NULL,
  nebula TEXT NOT NULL,
  galaxy_id UUID NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (galaxy_id) REFERENCES galaxies(id),
  UNIQUE (name, galaxy_id)
);

CREATE TABLE planets (
  id UUID NOT NULL DEFAULT gen_random_uuid(),
  name TEXT NOT NULL,
  capacity INT NOT NULL,
  star_id UUID,
  galaxy_id UUID NOT NULL,
  PRIMARY KEY (id),
  FOREIGN KEY (star_id) REFERENCES stars(id),
  FOREIGN KEY (galaxy_id) REFERENCES galaxies(id),
  UNIQUE (name, galaxy_id)
);