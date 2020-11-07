-- Your SQL goes here
CREATE TABLE users (
  uuid uuid NOT NULL,
  email character varying NOT NULL,
  password character(60),
  PRIMARY KEY (uuid)
);

CREATE TABLE objective (
  uuid uuid NOT NULL,
  name character varying NOT NULL,
  start_at timestamp without time zone,
  date_created timestamp NOT NULL DEFAULT NOW(),
  PRIMARY KEY (uuid)
);