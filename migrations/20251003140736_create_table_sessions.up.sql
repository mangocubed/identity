CREATE TABLE sessions (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL,
    token citext NOT NULL,
    user_agent varchar NOT NULL,
    country_alpha2 varchar NULL,
    region varchar NULL,
    city varchar NULL,
    finished_at timestamptz NULL,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_sessions PRIMARY KEY (id),
    CONSTRAINT fkey_sessions_to_users FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE UNIQUE INDEX index_sessions_on_token ON sessions USING btree (token);

SELECT manage_updated_at('sessions');
SELECT manage_versions('sessions');
