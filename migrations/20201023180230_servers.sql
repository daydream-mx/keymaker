CREATE TYPE registration AS ENUM ('open', 'invite', 'closed');

CREATE TABLE servers (
    id SERIAL PRIMARY Key,
    name text,
    url text,
    logo_url text,
    admins text[],
    categories text[], -- Dont link tables to prevent infinite loop!
    rules text,
    description text,
    registration_status registration
);