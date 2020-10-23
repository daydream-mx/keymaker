CREATE TABLE categories (
    id SERIAL PRIMARY KEY NOT NULL,
    name text NOT NULL,
    servers integer[] NOT NULL -- Foreign keys on array values are apparently not possible so we just keep the Ids
);