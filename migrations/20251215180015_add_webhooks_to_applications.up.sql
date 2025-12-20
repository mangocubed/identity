ALTER TABLE applications ADD COLUMN webhook_url varchar NULL, ADD COLUMN webhook_secret varchar NOT NULL DEFAULT '';
