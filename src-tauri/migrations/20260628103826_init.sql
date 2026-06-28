-- Add migration script here
CREATE TABLE portfolios (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    base_currency TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE accounts (
    id TEXT PRIMARY KEY,
    portfolio_id TEXT NOT NULL,

    name TEXT NOT NULL,
    institution TEXT,
    account_type TEXT NOT NULL,

    currency TEXT NOT NULL,

    is_active INTEGER NOT NULL DEFAULT 1,

    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (portfolio_id)
        REFERENCES portfolios(id)
        ON DELETE CASCADE
);

CREATE TABLE assets (
    id TEXT PRIMARY KEY,

    asset_type TEXT NOT NULL,

    name TEXT NOT NULL,
    symbol TEXT NOT NULL,
    exchange TEXT,
    currency TEXT NOT NULL,

    isin TEXT,

    is_active INTEGER NOT NULL DEFAULT 1,

    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,

    UNIQUE(symbol, exchange)
);

CREATE TABLE asset_identifiers (
    id TEXT PRIMARY KEY,

    asset_id TEXT NOT NULL,

    provider TEXT NOT NULL,
    identifier TEXT NOT NULL,

    FOREIGN KEY(asset_id)
        REFERENCES assets(id),

    UNIQUE(provider, identifier)
);
