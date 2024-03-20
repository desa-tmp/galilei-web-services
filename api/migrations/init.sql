CREATE TABLE users (
  id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL
);

CREATE TABLE sessions (
  id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
  expires TIMESTAMP NOT NULL,
  user_id UUID NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE galaxies (
  id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT NOT NULL,
  user_id UUID NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
  CONSTRAINT galaxy_name_user UNIQUE (name, user_id) -- unique galaxy name for a user
);

CREATE TABLE stars (
  id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT NOT NULL,
  nebula TEXT NOT NULL,
  galaxy_id UUID NOT NULL,
  FOREIGN KEY (galaxy_id) REFERENCES galaxies(id) ON DELETE CASCADE,
  CONSTRAINT star_name_galaxy UNIQUE (name, galaxy_id) -- unique star name inside a galaxy
);

CREATE TABLE planets (
  id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
  name TEXT NOT NULL,
  capacity INT NOT NULL, -- unit size is MB
  star_id UUID, -- a planet may not have a star associated with it
  galaxy_id UUID NOT NULL,
  FOREIGN KEY (star_id) REFERENCES stars(id) ON DELETE SET NULL,
  FOREIGN KEY (galaxy_id) REFERENCES galaxies(id) ON DELETE CASCADE,
  CONSTRAINT planet_name_galaxy UNIQUE (name, galaxy_id) -- unique planet name inside a galaxy
);