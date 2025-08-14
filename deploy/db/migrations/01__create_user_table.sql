
CREATE TABLE IF NOT EXISTS users
(
    id                    BIGSERIAL PRIMARY KEY,
    first_name            VARCHAR(100)             NOT NULL,
    last_name             VARCHAR(100)             NOT NULL,
    email                 VARCHAR(255)             NOT NULL
        CONSTRAINT proper_email CHECK (email ~* '^[A-Za-z0-9._%-]+@[A-Za-z0-9.-]+[.][A-Za-z]+$'),
    password              VARCHAR(255)             NOT NULL,
    role                  VARCHAR(20)              NOT NULL DEFAULT 'User',
    created_at            TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
    updated_at            TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
    last_login_at         TIMESTAMP WITH TIME ZONE,
    deleted_at            TIMESTAMP WITH TIME ZONE,
    deleted               BOOLEAN                  NOT NULL DEFAULT FALSE,
    is_verified           BOOLEAN                  NOT NULL DEFAULT FALSE,
    failed_login_attempts INTEGER                  NOT NULL DEFAULT 0
);

-- Create indexes for performance
CREATE UNIQUE INDEX IF NOT EXISTS idx_users_email ON users (email) WHERE deleted = false;
CREATE INDEX IF NOT EXISTS idx_users_role ON users (role);
CREATE INDEX IF NOT EXISTS idx_users_created_at ON users (created_at);

-- Create trigger to update updated_at automatically
CREATE OR REPLACE FUNCTION update_user_modified()
    RETURNS TRIGGER AS
$$
BEGIN
    NEW.updated_at = now() AT TIME ZONE 'utc';
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_user_modified
    BEFORE UPDATE
    ON users
    FOR EACH ROW
EXECUTE FUNCTION update_user_modified();

-- Create function for soft deletion
CREATE OR REPLACE FUNCTION soft_delete_user(user_id BIGINT)
    RETURNS VOID AS
$$
BEGIN
    UPDATE users
    SET deleted    = TRUE,
        deleted_at = now() AT TIME ZONE 'utc'
    WHERE id = user_id;
END;
$$ LANGUAGE plpgsql;
