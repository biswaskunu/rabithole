-- Migration: 001_initial_schema
-- Based on docs/Architecture.md Section 3 data model

CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS topics (
    id SERIAL PRIMARY KEY,
    slug VARCHAR(255) UNIQUE NOT NULL,
    title VARCHAR(255) NOT NULL,
    lang VARCHAR(10) DEFAULT 'en',
    summary TEXT,
    source_url VARCHAR(500),
    cached_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS topic_edges (
    from_topic_id INTEGER REFERENCES topics(id) ON DELETE CASCADE,
    to_topic_id INTEGER REFERENCES topics(id) ON DELETE CASCADE,
    relation_type VARCHAR(50) DEFAULT 'related',
    PRIMARY KEY (from_topic_id, to_topic_id)
);

CREATE TABLE IF NOT EXISTS maps (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    is_public BOOLEAN DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS map_nodes (
    id SERIAL PRIMARY KEY,
    map_id INTEGER REFERENCES maps(id) ON DELETE CASCADE,
    topic_id INTEGER REFERENCES topics(id),
    position_x FLOAT DEFAULT 0,
    position_y FLOAT DEFAULT 0,
    expanded_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS quiz_questions (
    id SERIAL PRIMARY KEY,
    topic_id INTEGER REFERENCES topics(id),
    question TEXT NOT NULL,
    options JSONB DEFAULT '[]'::jsonb,
    correct_option INTEGER DEFAULT 1
);

CREATE TABLE IF NOT EXISTS streaks (
    id INTEGER PRIMARY KEY DEFAULT 1,
    user_id INTEGER UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    current_streak INTEGER DEFAULT 0,
    longest_streak INTEGER DEFAULT 0,
    last_active_date DATE DEFAULT CURRENT_DATE
);

CREATE TABLE IF NOT EXISTS daily_trails (
    date DATE PRIMARY KEY,
    topic_ids INTEGER[] DEFAULT '{}'::integer[],
    generated_at TIMESTAMPTZ DEFAULT NOW()
);
