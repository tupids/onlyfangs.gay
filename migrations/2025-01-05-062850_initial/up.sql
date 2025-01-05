CREATE TYPE application_status AS ENUM ('pending', 'approved', 'maybe', 'rejected', 'in');

CREATE TYPE twitch_account_type AS ENUM ('pleb', 'affiliate', 'partner');

CREATE TABLE applications (
    id SERIAL NOT NULL PRIMARY KEY,
    twitch_id INT NOT NULL,
    twitch_username TEXT NOT NULL,
    twitch_display_name TEXT NOT NULL,
    twitch_profile_image_url TEXT NOT NULL,
    twitch_account_type twitch_account_type NOT NULL DEFAULT 'pleb',
    status application_status NOT NULL DEFAULT 'pending',
    reason TEXT NOT NULL CHECK (LENGTH(reason) <= 1000),
    support_clip_url TEXT NOT NULL CHECK (LENGTH(support_clip_url) <= 1000),
    follow_count INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

CREATE TABLE application_comments (
    id SERIAL PRIMARY KEY,
    application_id INT NOT NULL REFERENCES applications (id),
    comment TEXT NOT NULL CHECK (LENGTH(comment) <= 1000),
    twitch_user_id INT NOT NULL,
    twitch_username TEXT NOT NULL,
    twitch_display_name TEXT NOT NULL,
    twitch_profile_image_url TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX ON applications (status, twitch_account_type, follow_count, created_at);

-- Currently, we only support one application per twitch account.
-- We dont make this the primary key because this constraint may
-- be lifted in the future.
CREATE UNIQUE INDEX ON applications (twitch_id);

CREATE INDEX ON application_comments (application_id, created_at);
