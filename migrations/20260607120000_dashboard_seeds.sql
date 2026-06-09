-- Migration: dashboard_seeds
-- Purpose: Populate with realistic mock data for dashboard testing.

-- Add an Investor user
-- Dev seed: investor@assets.local / Senha@12345678
INSERT INTO users (id, name, email, password_hash, role, active, created_at)
VALUES (
    '00000000-0000-4000-8000-000000000002',
    'Eduardo Skokado',
    'investor@assets.local',
    '$2b$12$bFSrLmoJVBlhzViZypM9OuYaerhHwLqSkn998eU/woN1dW7boe8Z.',
    'investor',
    TRUE,
    NOW()
) ON CONFLICT (email) DO NOTHING;

-- Add more assets
INSERT INTO assets (id, ticker, name, asset_type, currency, active, created_at, updated_at)
VALUES
    (
        '00000000-0000-4000-8000-000000000103',
        'VALE3',
        'Vale ON',
        'STOCK',
        'BRL',
        TRUE,
        NOW(),
        NOW()
    ),
    (
        '00000000-0000-4000-8000-000000000104',
        'WEGE3',
        'Weg ON',
        'STOCK',
        'BRL',
        TRUE,
        NOW(),
        NOW()
    ),
    (
        '00000000-0000-4000-8000-000000000105',
        'IVVB11',
        'iShares S&P 500 Fundo de Índice',
        'ETF',
        'BRL',
        TRUE,
        NOW(),
        NOW()
    )
ON CONFLICT (ticker) DO NOTHING;

-- Add trades for the investor
INSERT INTO trades (id, user_id, asset_id, side, quantity, unit_price, currency, traded_at, created_at)
VALUES
    (gen_random_uuid(), '00000000-0000-4000-8000-000000000002', '00000000-0000-4000-8000-000000000101', 'BUY', 100, 35.50, 'BRL', NOW() - INTERVAL '30 days', NOW()),
    (gen_random_uuid(), '00000000-0000-4000-8000-000000000002', '00000000-0000-4000-8000-000000000102', 'BUY', 50, 160.00, 'BRL', NOW() - INTERVAL '25 days', NOW()),
    (gen_random_uuid(), '00000000-0000-4000-8000-000000000002', '00000000-0000-4000-8000-000000000103', 'BUY', 200, 65.20, 'BRL', NOW() - INTERVAL '20 days', NOW()),
    (gen_random_uuid(), '00000000-0000-4000-8000-000000000002', '00000000-0000-4000-8000-000000000104', 'BUY', 300, 42.00, 'BRL', NOW() - INTERVAL '15 days', NOW()),
    (gen_random_uuid(), '00000000-0000-4000-8000-000000000002', '00000000-0000-4000-8000-000000000105', 'BUY', 10, 250.00, 'BRL', NOW() - INTERVAL '10 days', NOW()),
    (gen_random_uuid(), '00000000-0000-4000-8000-000000000002', '00000000-0000-4000-8000-000000000101', 'SELL', 50, 38.00, 'BRL', NOW() - INTERVAL '5 days', NOW());

-- Add positions for the investor (normally updated by a trigger or job, but for seeding we insert manually)
INSERT INTO positions (user_id, asset_id, quantity, average_price, currency, updated_at)
VALUES
    ('00000000-0000-4000-8000-000000000002', '00000000-0000-4000-8000-000000000101', 50, 35.50, 'BRL', NOW()),
    ('00000000-0000-4000-8000-000000000002', '00000000-0000-4000-8000-000000000102', 50, 160.00, 'BRL', NOW()),
    ('00000000-0000-4000-8000-000000000002', '00000000-0000-4000-8000-000000000103', 200, 65.20, 'BRL', NOW()),
    ('00000000-0000-4000-8000-000000000002', '00000000-0000-4000-8000-000000000104', 300, 42.00, 'BRL', NOW()),
    ('00000000-0000-4000-8000-000000000002', '00000000-0000-4000-8000-000000000105', 10, 250.00, 'BRL', NOW())
ON CONFLICT (user_id, asset_id) DO UPDATE SET
    quantity = EXCLUDED.quantity,
    average_price = EXCLUDED.average_price,
    updated_at = NOW();
