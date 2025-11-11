ALTER TABLE sessions ADD COLUMN previous_token citext NULL,
ADD COLUMN expires_at timestamptz NOT NULL DEFAULT current_timestamp + interval '30 days',
ADD COLUMN refreshed_at timestamptz NULL;

CREATE UNIQUE INDEX index_sessions_on_previous_token ON sessions USING btree (previous_token);
