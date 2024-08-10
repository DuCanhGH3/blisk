-- Add up migration script here
CREATE TYPE PREACT AS ENUM (
    'like',
    'love',
    'laugh',
    'wow',
    'sad',
    'angry'
);

CREATE TABLE IF NOT EXISTS post_reactions (
    "type" PREACT NOT NULL,
    "user_id" BIGINT NOT NULL,
    "post_id" BIGINT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (post_id) REFERENCES posts (id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, post_id)
);

CREATE TABLE IF NOT EXISTS comment_reactions (
    "type" PREACT NOT NULL,
    "user_id" BIGINT NOT NULL,
    "comment_id" BIGINT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (comment_id) REFERENCES comments (id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, comment_id)
);