CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    last_name VARCHAR(255) NOT NULL,
    first_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    phone_number VARCHAR(255) UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    is_activated BOOLEAN NOT NULL DEFAULT FALSE,
    password_reset_token UUID,
    password_reset_expires_at TIMESTAMPTZ,
    account_activation_token UUID,
    account_activation_token_expires_at TIMESTAMPTZ,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
)