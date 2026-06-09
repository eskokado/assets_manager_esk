CREATE TABLE users (
    id UUID PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(254) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(20) NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users (email);

-- Dev seed: admin@assets.local / Senha@12345678
INSERT INTO users (id, name, email, password_hash, role, active, created_at)
VALUES (
    '00000000-0000-4000-8000-000000000001',
    'Admin',
    'admin@assets.local',
    '$2b$12$bFSrLmoJVBlhzViZypM9OuYaerhHwLqSkn998eU/woN1dW7boe8Z.',
    'admin',
    TRUE,
    NOW()
) ON CONFLICT (email) DO NOTHING;
