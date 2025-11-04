CREATE TYPE confirmation_action AS ENUM ('email', 'login', 'password_reset');

CREATE TABLE confirmations (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL,
    action confirmation_action NOT NULL,
    encrypted_code varchar NOT NULL,
    pending_attempts smallint NOT NULL DEFAULT 3,
    expires_at timestamptz NOT NULL DEFAULT current_timestamp + interval '1 hour',
    finished_at timestamptz NULL,
    canceled_at timestamptz NULL,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_confirmations PRIMARY KEY (id),
    CONSTRAINT fkey_confirmations_to_users FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE UNIQUE INDEX index_confirmations_on_user_id_action ON confirmations USING btree (user_id, action)
WHERE finished_at IS NULL AND canceled_at IS NULL;

SELECT manage_updated_at('confirmations');
SELECT manage_versions('confirmations');
