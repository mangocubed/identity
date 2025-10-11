CREATE TABLE authorizations (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    application_id uuid NOT NULL,
    user_id uuid NOT NULL,
    session_id uuid NOT NULL,
    token citext NOT NULL,
    previous_token citext NULL,
    expires_at timestamptz NOT NULL DEFAULT current_timestamp + interval '1 hour',
    refreshed_at timestamptz NULL,
    revoked_at timestamptz NULL,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_authorizations PRIMARY KEY (id),
    CONSTRAINT fkey_authorizations_to_applications FOREIGN KEY (application_id) REFERENCES applications (id)
    ON DELETE CASCADE,
    CONSTRAINT fkey_authorizations_to_users FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    CONSTRAINT fkey_authorizations_to_sessions FOREIGN KEY (session_id) REFERENCES sessions (id)
    ON DELETE CASCADE
);

CREATE UNIQUE INDEX index_authorizations_on_application_id_user_id_session_id ON authorizations
USING btree (application_id, user_id, session_id);
CREATE UNIQUE INDEX index_authorizations_on_token ON authorizations USING btree (token);

SELECT manage_updated_at('authorizations');
SELECT manage_versions('authorizations');
