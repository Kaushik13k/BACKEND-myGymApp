-- Your SQL goes here
-- This script only contains the table creation statements and does not fully represent the table in the database. It's still missing: indices, triggers. Do not use it as a backup.

-- Sequence and defined type
CREATE SEQUENCE IF NOT EXISTS muscle_group_id_seq;

-- Table Definition
CREATE TABLE "public"."muscle_group" (
    "id" int4 NOT NULL DEFAULT nextval('muscle_group_id_seq'::regclass),
    "muscle_group_name" varchar(255),
    "muscle_group_image" varchar(255),
    PRIMARY KEY ("id")
);