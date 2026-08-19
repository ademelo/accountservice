CREATE TABLE IF NOT EXISTS orders
(
    id                  SERIAL PRIMARY KEY,
    order_datetime      TIMESTAMP   NOT NULL,
    order_total_value   MONEY       NOT NULL,
    client_id           INTEGER     NOT NULL,
    CONSTRAINT fk_orders_users FOREIGN KEY (client_id) REFERENCES users(id) ON DELETE CASCADE
);
