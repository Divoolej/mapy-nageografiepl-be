CREATE TABLE teachers (
  id SERIAL PRIMARY KEY,
  email VARCHAR NOT NULL,
  password_digest VARCHAR NOT NULL,
  auth_token VARCHAR
);

CREATE UNIQUE INDEX teachers_unique_email ON teachers(email);