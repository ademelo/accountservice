CREATE TABLE IF NOT EXISTS product
(
    id              SERIAL PRIMARY KEY,
    product_name    VARCHAR(36)   NOT NULL,
    product_price   MONEY         NULL
);
