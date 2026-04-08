CREATE TABLE application_tokens (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    application_id uuid NOT NULL,
    name citext NOT NULL,
    code citext NOT NULL,
    expires_at timestamptz NOT NULL DEFAULT current_timestamp + interval '1 year',
    revoked_at timestamptz NULL,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_application_tokens PRIMARY KEY (id),
    CONSTRAINT fkey_application_tokens_to_applications FOREIGN KEY (application_id) REFERENCES applications (id)
    ON DELETE CASCADE
);

CREATE UNIQUE INDEX index_application_tokens_on_application_id_name ON application_tokens
USING btree (application_id, name);
CREATE UNIQUE INDEX index_application_tokens_on_code ON application_tokens USING btree (code);

SELECT manage_updated_at('application_tokens');
SELECT manage_versions('application_tokens');
