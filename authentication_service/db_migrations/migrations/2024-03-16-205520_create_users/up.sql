-- Your SQL goes here
-- Sequence and defined type
CREATE SEQUENCE IF NOT EXISTS users_id_seq;

-- Table Definition
CREATE TABLE "public"."users" (
    "id" int4 NOT NULL DEFAULT nextval('users_id_seq'::regclass),
    "firstname" varchar,
    "lastname" varchar,
    "username" varchar,
    "email" varchar,
    "hash" varchar,
    "phone_number" int4,
    "age" int4,
    PRIMARY KEY ("id")
);