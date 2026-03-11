ALTER TABLE access_tokens
ADD COLUMN application_id uuid NOT NULL,
ADD COLUMN session_id uuid NOT NULL,
ADD COLUMN user_id uuid NOT NULL,
ADD CONSTRAINT fkey_access_tokens_to_applications FOREIGN KEY (application_id) REFERENCES applications (id)
ON DELETE CASCADE,
ADD CONSTRAINT fkey_access_tokens_to_sessions FOREIGN KEY (session_id) REFERENCES sessions (id) ON DELETE CASCADE,
ADD CONSTRAINT fkey_access_tokens_to_users FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE;

ALTER TABLE authorizations ADD COLUMN user_id uuid NOT NULL,
ADD CONSTRAINT fkey_authorizations_to_users FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE;

CREATE UNIQUE INDEX index_authorizations_on_application_id_user_id ON authorizations
USING btree (application_id, user_id);

DROP INDEX IF EXISTS index_authorizations_on_application_id_session_id;
