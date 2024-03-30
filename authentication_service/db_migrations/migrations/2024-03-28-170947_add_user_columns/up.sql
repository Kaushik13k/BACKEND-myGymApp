-- Your SQL goes here
ALTER TABLE users
RENAME COLUMN age TO dob;

ALTER TABLE users
ADD COLUMN height int4; 