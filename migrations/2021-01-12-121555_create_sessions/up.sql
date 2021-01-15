CREATE TABLE sessions (
  id SERIAL PRIMARY KEY,
  uuid VARCHAR NOT NULL,
  owner_type VARCHAR NOT NULL,
  owner_uuid VARCHAR NOT NULL,
  refresh_token VARCHAR NOT NULL,
  refresh_token_expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
  access_token VARCHAR NOT NULL,
  access_token_expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX sessions_owner_uuid ON sessions(owner_uuid);
CREATE UNIQUE INDEX sessions_unique_uuid ON sessions(uuid);
CREATE UNIQUE INDEX sessions_unique_access_token ON sessions(access_token);
CREATE UNIQUE INDEX sessions_unique_refresh_token ON sessions(refresh_token);

SELECT diesel_manage_updated_at('sessions');