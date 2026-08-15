CREATE TABLE IF NOT EXISTS users
(
    id          SERIAL PRIMARY KEY,
    first_name  VARCHAR(36)   NOT NULL,
    last_name   VARCHAR(36)    NULL,
    country     VARCHAR(36)    NULL
);
