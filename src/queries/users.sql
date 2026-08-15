-- queries/users.sql

--! insert_users
INSERT INTO users
(first_name, last_name, country)
VALUES
    (:first_name, :last_name, :country);

--! users
SELECT first_name, last_name, country FROM users;