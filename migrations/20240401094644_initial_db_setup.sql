CREATE TABLE IF NOT EXISTS quotes (
    id UUID PRIMARY KEY,
    book VARCHAR NOT NULL,
    quote TEXT NOT NULL,
    inserted_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    UNIQUE (book, quote)
);
