-- Add migration script here
CREATE TABLE IF NOT EXISTS bids (
    id SERIAL PRIMARY KEY,
    team_id TEXT NOT NULL,
    amount REAL NOT NULL,
    timestamp BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS locations (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    location TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS topics (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS pools (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    created_at BIGINT NOT NULL,
    description TEXT,
    is_active BOOLEAN,
    owner_id INTEGER NOT NULL,
    location_id INTEGER,
    topic_id INTEGER,
    max_participants INTEGER,
    FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (location_id) REFERENCES locations(id) ON DELETE SET NULL,
    FOREIGN KEY (topic_id) REFERENCES topics(id) ON DELETE CASCADE
);
