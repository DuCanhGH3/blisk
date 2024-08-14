-- Add down migration script here
DROP TABLE IF EXISTS oauth_client_contacts, oauth_client_redirect, oauth_client_name, oauth_client;

DROP TYPE OACTYPE, OACAUTHMETHOD;
