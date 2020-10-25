CREATE TABLE servers_categories (
    server_url    text REFERENCES servers (url) ON UPDATE CASCADE ON DELETE CASCADE,
    category_name text REFERENCES categories (name) ON UPDATE CASCADE ON DELETE CASCADE,
    CONSTRAINT servers_categories_pkey PRIMARY KEY (server_url, category_name)  -- explicit pk
);
