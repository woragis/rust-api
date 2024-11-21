CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,                           -- Unique identifier for each user
    first_name VARCHAR(100) NOT NULL,               -- User's first name
    last_name VARCHAR(100) NOT NULL,                -- User's last name
    email VARCHAR(255) UNIQUE NOT NULL,             -- Email address for login
    password_hash TEXT NOT NULL,                    -- Hashed password for security
    profile_picture TEXT,                           -- URL for the profile picture
    phone_number VARCHAR(20),                       -- Phone number
    address JSONB,                                  -- JSON object for storing address details
    preferences JSONB,                              -- JSON object for user preferences/settings
    is_verified BOOLEAN DEFAULT FALSE,              -- Whether the user has verified their email/phone
    role VARCHAR(50) DEFAULT 'user',                -- Role of the user (e.g., user, admin)
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- Timestamp when the user account was created
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP, -- Last update timestamp
    last_login TIMESTAMP                            -- Timestamp of the user's last login
);

SELECT * FROM users;

SELECT * FROM users WHERE id = $1;

INSERT INTO users (
    first_name,
    last_name,
    email,
    password_hash,
    profile_picture,
    phone_number,
    address,
    preferences,
    is_verified,
    role,
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
    $9,
    $10,
    $11
);

UPDATE users SET
    first_name = $1,
    last_name = $2,
    email = $3,
    password_hash = $4,
    profile_picture = $5,
    phone_number = $6,
    address = $7,
    preferences = $8,
    is_verified = $9,
    role = $10,
    last_login = $11
    WHERE id = $12
);

DELETE FROM users WHERE id = $1;
