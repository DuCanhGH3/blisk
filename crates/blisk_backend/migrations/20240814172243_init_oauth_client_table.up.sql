-- Add up migration script here
CREATE TYPE OACAUTHMETHOD AS ENUM ('client_secret_basic', 'client_secret_post', 'client_secret_jwt', 'private_key_jwt', 'none');

CREATE TYPE OACTYPE AS ENUM ('web', 'native');

CREATE TABLE IF NOT EXISTS oauth_client (
  "id" BIGSERIAL NOT NULL PRIMARY KEY,
  "owner" BIGINT NOT NULL,
  "type" OACTYPE NOT NULL,
  "endpoint_auth" OACAUTHMETHOD NOT NULL,
  "secret" TEXT,
  "logo_uri" TEXT NOT NULL,
  "userinfo_alg" TEXT NOT NULL,
  "userinfo_enc" TEXT NOT NULL,
  FOREIGN KEY ("owner") REFERENCES users ("id") ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS oauth_client_name (
  "id" BIGSERIAL NOT NULL PRIMARY KEY,
  "client_id" BIGINT NOT NULL,
  "lang" TEXT,
  "name" TEXT NOT NULL,
  FOREIGN KEY ("client_id") REFERENCES oauth_client ("id") ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS oauth_client_redirect (
  "id" BIGSERIAL NOT NULL PRIMARY KEY,
  "client_id" BIGINT NOT NULL,
  "redirect_uri" TEXT,
  FOREIGN KEY ("client_id") REFERENCES oauth_client ("id") ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS oauth_client_contacts (
  "id" BIGSERIAL NOT NULL PRIMARY KEY,
  "client_id" BIGINT NOT NULL,
  "email" TEXT NOT NULL,
  FOREIGN KEY ("client_id") REFERENCES oauth_client ("id") ON DELETE CASCADE
);