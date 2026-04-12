-- Stupide Test database.


-- Parent Table (Primary Key)
CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT UNIQUE -- Unique Constraint
);

-- Child Table (Foreign Key + Generated Column)
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    category_id INTEGER REFERENCES categories(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    price NUMERIC(10, 2) NOT NULL,
    tax_rate NUMERIC(3, 2) DEFAULT 0.20,
    -- Stored/Generated Column (Postgres 12+)
    total_price NUMERIC(10, 2) GENERATED ALWAYS AS (price * (1 + tax_rate)) STORED
);

-- A Stored Procedure
CREATE OR REPLACE PROCEDURE add_category(cat_name TEXT, cat_code TEXT)
LANGUAGE plpgsql
AS $$
BEGIN
    INSERT INTO categories (name, code) VALUES (cat_name, cat_code);
END;
$$;

-- Insert dummy data for testing
CALL add_category('Electronics', 'ELEC');
INSERT INTO products (category_id, title, price) VALUES (1, 'Testing Widget', 100.00);

-- Verify the data exists
SELECT * FROM products;
