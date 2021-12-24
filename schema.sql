CREATE EXTENSION IF NOT EXISTS citext;

CREATE DOMAIN mytext AS CITEXT
    CHECK ( VALUE != '' );

CREATE TYPE my_enum AS ENUM ( 'state1', 'state2', 'state3' );

CREATE TABLE table1 (
    table1_id  UUID  PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    text  TEXT  NOT NULL  DEFAULT 'a',
    text_null  TEXT  NULL,
    citext  CITEXT  NOT NULL  DEFAULT 'b',
    citext_null  CITEXT  NULL,
    mytext  mytext  NOT NULL  DEFAULT 'c',
    mytext_null  mytext  NULL,
    myenum  my_enum  NOT NULL  DEFAULT 'state1',
    myenum_null  my_enum  NULL
);
