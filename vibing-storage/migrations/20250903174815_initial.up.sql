-- Add up migration script here
-- CREATE EXTENSION IF NOT EXISTS CITEXT;

CREATE TABLE tracks (
    track_id SERIAL PRIMARY KEY,
    path VARCHAR(255) UNIQUE NOT NULL,
    title TEXT NULL,
    author TEXT NULL,
    genre TEXT NULL,
    duration INT NULL,
    vote_count INT NOT NULL DEFAULT 0,
    total_rating BIGINT NOT NULL DEFAULT 0,
    download_count INT NOT NULL DEFAULT 0
);

CREATE TABLE vibe_groups (
    vibe_group_id SERIAL PRIMARY KEY,
    name CITEXT UNIQUE NOT NULL
);

CREATE TABLE vibes (
    vibe_id SERIAL PRIMARY KEY,
    name CITEXT NOT NULL,
    vibe_group INT NOT NULL REFERENCES vibe_groups(vibe_group_id) ON DELETE RESTRICT ON UPDATE CASCADE,

    CONSTRAINT unique_vibe UNIQUE(name, vibe_group) DEFERRABLE INITIALLY IMMEDIATE
);

CREATE TABLE tracks_with_vibes (
    track INT NOT NULL REFERENCES tracks(track_id) ON DELETE CASCADE ON UPDATE CASCADE,
    vibe INT NOT NULL REFERENCES vibes(vibe_id) ON DELETE CASCADE ON UPDATE CASCADE,
    PRIMARY KEY (track, vibe)
);

GRANT TRIGGER, REFERENCES, SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA public TO viber;
GRANT USAGE ON ALL SEQUENCES IN SCHEMA public TO viber;