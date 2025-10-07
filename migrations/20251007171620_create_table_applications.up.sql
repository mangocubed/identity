CREATE TABLE applications (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    name citext NOT NULL,
    redirect_url varchar NOT NULL,
    encrypted_secret varchar NOT NULL,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_applications PRIMARY KEY (id)
);

CREATE UNIQUE INDEX index_applications_on_name ON applications USING btree (name);

SELECT manage_updated_at('applications');
SELECT manage_versions('applications');
