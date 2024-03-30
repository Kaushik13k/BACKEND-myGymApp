-- Your SQL goes here
CREATE SEQUENCE IF NOT EXISTS body_measurements_id_seq;

CREATE TABLE "public"."body_measurements" (
    "id" int4 NOT NULL DEFAULT nextval('body_measurements_id_seq'::regclass),
    "user_id" int4 NOT NULL,
    "weight" float,
    "height" float,
    "weist" float,
    "neck" float, 
    "shoulders" float,
    "chest" float,
    "left_bicep" float,
    "right_bicep" float,
    "left_forearm" float,
    "right_forearm" float,
    "abdomen" float,
    "hips" float,
    "left_thigh" float,
    "right_thigh" float,
    "left_calf" float,
    "right_calf" float,    
    PRIMARY KEY ("id"),
    FOREIGN KEY ("user_id") REFERENCES "public"."users"("id")
);
