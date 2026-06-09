CREATE TABLE assets (
    id UUID PRIMARY KEY,
    ticker VARCHAR(20) NOT NULL UNIQUE,
    name VARCHAR(200) NOT NULL,
    asset_type VARCHAR(20) NOT NULL,
    currency CHAR(3) NOT NULL DEFAULT 'BRL',
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_assets_ticker ON assets (ticker);
CREATE INDEX idx_assets_active ON assets (active);
CREATE INDEX idx_assets_asset_type ON assets (asset_type);

INSERT INTO assets (id, ticker, name, asset_type, currency, active, created_at, updated_at)
VALUES
    (
        '00000000-0000-4000-8000-000000000101',
        'PETR4',
        'Petrobras PN',
        'STOCK',
        'BRL',
        TRUE,
        NOW(),
        NOW()
    ),
    (
        '00000000-0000-4000-8000-000000000102',
        'HGLG11',
        'CSHG Logística FII',
        'FII',
        'BRL',
        TRUE,
        NOW(),
        NOW()
    )
ON CONFLICT (ticker) DO NOTHING;
