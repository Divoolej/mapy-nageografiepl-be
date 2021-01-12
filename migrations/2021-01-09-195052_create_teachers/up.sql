CREATE TABLE teachers (
  id SERIAL PRIMARY KEY,
  uuid VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  password_digest VARCHAR NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX teachers_unique_uuid ON teachers(uuid);
CREATE UNIQUE INDEX teachers_unique_email ON teachers(email);

SELECT diesel_manage_updated_at('teachers');