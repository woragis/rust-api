CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    role VARCHAR(50) DEFAULT 'user' CHECK (role in ('user', 'admin')),
    profile_picture TEXT,
    phone_number VARCHAR(20),
    is_verified BOOLEAN DEFAULT FALSE,
    last_login TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

falar com miguel no insta;

SELECT * FROM users;

SELECT * FROM users WHERE id = $1;

INSERT INTO users (
    first_name,
    last_name,
    email,
    password,
    role,
    profile_picture,
    phone_number,
    is_verified,
    last_login
    ) VALUES (
    $1,
    $2,
    $3,
    $4,
    $5,
    $6,
    $7,
    $8,
    $9
);

UPDATE users SET
    first_name = $1,
    last_name = $2,
    email = $3,
    password = $4,
    role = $5,
    profile_picture = $6,
    phone_number = $7,
    is_verified = $8,
    last_login = $9
    WHERE id = $10
);

DELETE FROM users WHERE id = $1;
