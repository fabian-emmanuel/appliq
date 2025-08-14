

CREATE TABLE IF NOT EXISTS applications
(
    id               BIGSERIAL PRIMARY KEY,
    company          VARCHAR(70)                 NOT NULL,
    position         VARCHAR(100)                NOT NULL,
    website          VARCHAR(255),
    application_type VARCHAR(30)                 NOT NULL,
    created_by       BIGINT                      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    created_at       TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
    updated_at       TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
    deleted_at       TIMESTAMP WITH TIME ZONE,
    deleted          BOOLEAN                     NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS application_statuses
(
    id             BIGSERIAL PRIMARY KEY,
    application_id BIGINT                      NOT NULL REFERENCES applications (id) ON DELETE CASCADE,
    status_type    VARCHAR(30)                 NOT NULL,
    created_by     BIGINT                      NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    created_at     TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT (now() AT TIME ZONE 'utc'),
    test_type      VARCHAR(30),
    interview_type VARCHAR(30),
    notes          TEXT
);


-- =============================================
-- INDEXES
-- =============================================
CREATE INDEX IF NOT EXISTS idx_application_statuses_application_id ON application_statuses (application_id);
CREATE INDEX IF NOT EXISTS idx_application_statuses_status_type ON application_statuses (status_type);
CREATE INDEX IF NOT EXISTS idx_applications_company ON applications (company);
CREATE INDEX IF NOT EXISTS idx_applications_position ON applications (position);
CREATE INDEX IF NOT EXISTS idx_applications_deleted ON applications (deleted);
CREATE INDEX IF NOT EXISTS idx_applications_created_at ON applications (created_at);
CREATE INDEX IF NOT EXISTS idx_application_statuses_created_at ON application_statuses (created_at);
