-- Initial Migration for Creating Empty Database tables

-- Testing Purposes
CREATE TABLE todos (
    id            UUID       NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_on    TIMESTAMP  NOT NULL,
    description   TEXT       NOT NULL,
    done          BOOLEAN    NOT NULL DEFAULT FALSE
);

-- Actual CMS Tables
CREATE TABLE users (
    id            UUID       NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_on    TIMESTAMP  NOT NULL,
    email         TEXT       NOT NULL UNIQUE,
    password_hash TEXT       NOT NULL,
    name          TEXT       NOT NULL,
    roles         TEXT[] 
);

CREATE TABLE uploads (
    id            UUID       NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_on    TIMESTAMP  NOT NULL,
    created_by_id UUID       NOT NULL, -- relation(user)
    url           TEXT,
    data          JSONB,
    index0        TEXT,
    index1        TEXT,
    index2        TEXT,
    index3        TEXT,
    index4        TEXT,
    index5        TEXT
);

CREATE TABLE documents (
    id            UUID       NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_on    TIMESTAMP  NOT NULL,
    created_by_id UUID       NOT NULL, -- relation(users)
    name          TEXT,
    route_id      UUID       UNIQUE, -- relation(routes)
    data          JSONB,
    index0        TEXT,
    index1        TEXT,
    index2        TEXT,
    index3        TEXT,
    index4        TEXT,
    index5        TEXT
);

CREATE TYPE route_type AS ENUM ('document', 'redirect');

CREATE TABLE routes (
    id            UUID       NOT NULL DEFAULT gen_random_uuid() PRIMARY KEY,
    created_on    TIMESTAMP  NOT NULL,
    created_by_id UUID       NOT NULL, -- relation(users)
    type          route_type NOT NULL DEFAULT 'document',
    slug          TEXT       NOT NULL,
    domain        TEXT       NOT NULL,
    hash          TEXT       NOT NULL UNIQUE,
    document_id   UUID, -- relation(documents) 
    redirect_id   UUID -- relation(redirects) 
);
