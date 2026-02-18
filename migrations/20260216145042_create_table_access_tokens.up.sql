CREATE TABLE access_tokens (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    authorization_id uuid NOT NULL,
    code citext NOT NULL,
    refresh_code citext NOT NULL,
    code_expires_at timestamptz NOT NULL DEFAULT current_timestamp + interval '1 day',
    expires_at timestamptz NOT NULL DEFAULT current_timestamp + interval '30 days',
    refreshed_at timestamptz NULL,
    revoked_at timestamptz NULL,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_access_tokens PRIMARY KEY (id),
    CONSTRAINT fkey_access_tokens_to_authorizations FOREIGN KEY (authorization_id) REFERENCES authorizations (id)
    ON DELETE CASCADE
);

CREATE UNIQUE INDEX index_access_tokens_on_code ON access_tokens USING btree (code);
CREATE UNIQUE INDEX index_access_tokens_on_refresh_code ON access_tokens USING btree (refresh_code);

SELECT manage_updated_at('access_tokens');
SELECT manage_versions('access_tokens');
