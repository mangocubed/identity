CREATE TABLE sessions (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL,
    ip_address varchar(255) NULL,
    country_code varchar(2) NULL,
    region varchar(255) NULL,
    city varchar(255) NULL,
    expires_at timestamptz NOT NULL DEFAULT current_timestamp + interval '30 days',
    refreshed_at timestamptz NULL,
    finished_at timestamptz NULL,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_sessions PRIMARY KEY (id),
    CONSTRAINT fkey_sessions_to_users FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

SELECT manage_updated_at('sessions');
SELECT manage_versions('sessions');
