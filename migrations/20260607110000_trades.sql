CREATE TABLE trades (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users (id),
    asset_id UUID NOT NULL REFERENCES assets (id),
    side VARCHAR(4) NOT NULL CHECK (side IN ('BUY', 'SELL')),
    quantity NUMERIC(18, 8) NOT NULL CHECK (quantity > 0),
    unit_price NUMERIC(18, 8) NOT NULL CHECK (unit_price > 0),
    currency CHAR(3) NOT NULL DEFAULT 'BRL',
    traded_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_trades_user_id ON trades (user_id);
CREATE INDEX idx_trades_asset_id ON trades (asset_id);
CREATE INDEX idx_trades_user_traded_at ON trades (user_id, traded_at DESC);
CREATE INDEX idx_trades_side ON trades (side);

CREATE TABLE positions (
    user_id UUID NOT NULL REFERENCES users (id),
    asset_id UUID NOT NULL REFERENCES assets (id),
    quantity NUMERIC(18, 8) NOT NULL CHECK (quantity >= 0),
    average_price NUMERIC(18, 8) NOT NULL CHECK (average_price >= 0),
    currency CHAR(3) NOT NULL DEFAULT 'BRL',
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, asset_id)
);

CREATE INDEX idx_positions_asset_id ON positions (asset_id);
