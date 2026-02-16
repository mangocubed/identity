CREATE TABLE authorizations (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    application_id uuid NOT NULL,
    session_id uuid NOT NULL,
    redirect_url varchar NOT NULL,
    code citext NOT NULL,
    code_challenge citext NOT NULL,
    expires_at timestamptz NOT NULL DEFAULT current_timestamp + interval '10 minutes',
    revoked_at timestamptz NULL,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_authorizations PRIMARY KEY (id),
    CONSTRAINT fkey_authorizations_to_applications FOREIGN KEY (application_id) REFERENCES applications (id)
    ON DELETE CASCADE,
    CONSTRAINT fkey_authorizations_to_sessions FOREIGN KEY (session_id) REFERENCES sessions (id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX index_authorizations_on_application_id_session_id ON authorizations
USING btree (application_id, session_id);
CREATE UNIQUE INDEX index_authorizations_on_code ON authorizations USING btree (code);
CREATE UNIQUE INDEX index_authorizations_on_code_challenge ON authorizations USING btree (code_challenge);

SELECT manage_updated_at('authorizations');
SELECT manage_versions('authorizations');
