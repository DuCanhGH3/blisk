CREATE EXTENSION IF NOT EXISTS ltree;

CREATE EXTENSION IF NOT EXISTS fuzzystrmatch;

CREATE OR REPLACE FUNCTION immut_array_to_string(TEXT[]) 
RETURNS TEXT LANGUAGE sql IMMUTABLE AS $$SELECT array_to_string($1, ',')$$;

CREATE OR REPLACE FUNCTION construct_image(owner_id BIGINT, id BIGINT, ext TEXT)
RETURNS JSONB LANGUAGE sql IMMUTABLE AS $$SELECT jsonb_build_object('id', id, 'ext', ext, 'owner', owner_id)$$;