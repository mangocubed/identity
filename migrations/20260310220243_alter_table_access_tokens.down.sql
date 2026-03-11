ALTER TABLE authorizations DROP COLUMN user_id;

ALTER TABLE access_tokens DROP COLUMN application_id, DROP COLUMN session_id, DROP COLUMN user_id;
