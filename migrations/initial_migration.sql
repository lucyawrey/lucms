-- Initial Migration for Creating Empty Database

-- Testing Purposes
CREATE TABLE todos (
    id            UUID      NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    description   TEXT      NOT NULL,
    done          BOOLEAN   NOT NULL DEFAULT FALSE
);

-- Actual CMS Tables
-- CREATE TABLE users (
--     id            UUID    NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY
--     email         TEXT    NOT NULL UNIQUE
--     password_hash TEXT    NOT NULL
--     name          TEXT    NOT NULL
--     roles         TEXT[] 
-- )

-- CREATE TABLE documents (
--     id            UUID      NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
--     created_on    TIMESTAMP NOT NULL
--     created_by_id UUID      NOT NULL
--     name          TEXT
--     route_id      UUID      UNIQUE 
--     data          JSONB
--     index0        TEXT
--     index1        TEXT
--     index2        TEXT
--     index3        TEXT
--     index4        TEXT
--     index5        TEXT
-- );

-- CREATE TABLE routes (
--     id            UUID    NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY

--  type    enum(document, redirect) 
--  slug    TEXT 
--  domain    TEXT 
--  hash    TEXT(domain + slug)  UNIQUE 
--  document    Document(UUID)? 
--  redirect    Redirect(UUID)? 
-- )

-- CREATE TABLE uploads (
--     id            UUID    NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY

--  url    TEXT 
--  created_on    datetime 
--  created_by    User(UUID) 
--  data    json? 
-- )
