CREATE TABLE IF NOT EXISTS products (
    id BIGSERIAL PRIMARY KEY,                         -- Unique identifier for each product
    name VARCHAR(255) NOT NULL,                   -- Name of the product
    description TEXT,                             -- Detailed description of the product
    category VARCHAR(100),                        -- Category of the product (e.g., electronics, fashion)
    images TEXT[],                                -- Array of image URLs for the product
    price DECIMAL(10, 2) NOT NULL,                -- Price of the product
    discount DECIMAL(5, 2) DEFAULT 0.00,          -- Discount percentage (if any)
    currency VARCHAR(3) DEFAULT 'USD',            -- Currency code (e.g., USD, EUR)
    stock INT DEFAULT 0,                          -- Number of items available in stock
    weight DECIMAL(10, 2),                        -- Weight of the product in kilograms
    dimensions JSONB,                             -- JSON object for dimensions (length, width, height)
    tags TEXT[],                                  -- Array of tags for search optimization
    is_active BOOLEAN DEFAULT TRUE,               -- Whether the product is active and available
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Timestamp when the product is created
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP-- Last update timestamp
);

SELECT * FROM products;

SELECT * FROM products WHERE id = $1;

INSERT INTO products (
    name,
    description,
    category,
    images,
    price,
    discount,
    currency,
    stock,
    weight,
    dimensions,
    tags,
    is_active
    ) VALUES (
    $1,
    $2,
    $3,
    $4,
    $5,
    $6,
    $7,
    $8,
    $9,
    $10,
    $11,
    $12
);

UPDATE products SET
    name = $1,
    description = $2,
    category = $3,
    images = $4,
    price = $5,
    discount = $6,
    currency = $7,
    stock = $8,
    weight = $9,
    dimensions = $10,
    tags = $11,
    is_active = $12
    WHERE id = $13
);

DELETE FROM products WHERE id = $1;
