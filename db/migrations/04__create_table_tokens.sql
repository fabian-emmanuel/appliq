CREATE TABLE IF NOT EXISTS tokens
(
    id         BIGSERIAL PRIMARY KEY,
    user_id    BIGINT                   NOT NULL REFERENCES users (id),
    token      VARCHAR(100)             NOT NULL UNIQUE,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
    used       BOOLEAN                  NOT NULL DEFAULT FALSE
);

CREATE INDEX idx_tokens_token ON tokens (token);
CREATE INDEX idx_tokens_user_id ON tokens (user_id);