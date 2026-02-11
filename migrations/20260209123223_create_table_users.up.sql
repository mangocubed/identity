CREATE TABLE users (
    id uuid NOT NULL DEFAULT gen_random_uuid(),
    username citext NOT NULL,
    email citext NOT NULL,
    encrypted_password varchar NOT NULL,
    full_name varchar(255) NOT NULL,
    display_name varchar(255) NOT NULL,
    birthdate date NOT NULL,
    language_code varchar(2) NOT NULL DEFAULT 'en',
    country_code varchar(2) NOT NULL,
    disabled_at timestamptz NULL,
    created_at timestamptz NOT NULL DEFAULT current_timestamp,
    updated_at timestamptz NULL,
    CONSTRAINT pkey_users PRIMARY KEY (id),
    CONSTRAINT check_users_username
    CHECK (length(username) BETWEEN 3 AND 16 AND username ~ '^[-_.]?([[:alnum:]]+[-_.]?)+$'),
    CONSTRAINT check_users_email
    CHECK (length(email) BETWEEN 5 AND 255 AND email ~ '^[\w.-]+@[[:alnum:].-]+(\.[[:alpha:]]{2,})+$'),
    CONSTRAINT check_users_full_name CHECK (length(full_name) > 0),
    CONSTRAINT check_users_display_name CHECK (length(display_name) > 0),
    CONSTRAINT check_users_birthdate CHECK (birthdate <= current_date),
    CONSTRAINT check_users_language_code CHECK (language_code ~ '^[a-z]{2}$'),
    CONSTRAINT check_users_country_code CHECK (country_code ~ '^[A-Z]{2}$')
);

CREATE UNIQUE INDEX index_users_on_username ON users USING btree (username) WHERE username != '';
CREATE UNIQUE INDEX index_users_on_email ON users USING btree (email);

SELECT manage_updated_at('users');
SELECT manage_versions('users');
