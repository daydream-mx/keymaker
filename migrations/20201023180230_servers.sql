CREATE TYPE registration AS ENUM ('open', 'invite', 'closed');

CREATE TABLE servers (
    name text,
    url text PRIMARY KEY,
    server_name text, -- The part in a mxid to the right of the colons
    logo_url text,
    admins text[],
    categories text[], -- Dont link tables to prevent infinite loop!
    rules text,
    description text,
    registration_status registration
);