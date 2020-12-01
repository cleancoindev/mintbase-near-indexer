--
-- PostgreSQL database dump
--

-- Dumped from database version 12.4 (Debian 12.4-1.pgdg100+1)
-- Dumped by pg_dump version 12.2

-- Started on 2020-09-16 13:54:00 CEST

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

SET default_table_access_method = heap;



-- Table: public.stores

-- DROP TABLE public.stores;

CREATE TABLE public.stores
(
    id text COLLATE pg_catalog."default" NOT NULL,
    name text COLLATE pg_catalog."default" NOT NULL,
    symbol text COLLATE pg_catalog."default",
    "totalSupply" numeric,
    "tokenCount" numeric,
    "boughtCount" numeric,
    "transferCount" numeric,
    owner text COLLATE pg_catalog."default",
    "valueCount" numeric DEFAULT 0,
    "timestamp" timestamp without time zone,
    CONSTRAINT stores_pkey PRIMARY KEY (id)
)


ALTER TABLE public.stores
    OWNER to mintbase;

-- Table: public.things

-- DROP TABLE public.things;

CREATE TABLE public.things
(
    id text COLLATE pg_catalog."default" NOT NULL,
    minter text COLLATE pg_catalog."default",
    "timestamp" timestamp without time zone,
    burned boolean,
    "forSale" boolean,
    "metaId" text COLLATE pg_catalog."default",
    "resolveStore" text COLLATE pg_catalog."default",
    CONSTRAINT things_pkey PRIMARY KEY (id)
)


ALTER TABLE public.things
    OWNER to mintbase;


-- Table: public.tokens

-- DROP TABLE public.tokens;

CREATE TABLE public.tokens
(
    id text COLLATE pg_catalog."default" NOT NULL,
    "tokenId" text COLLATE pg_catalog."default",
    "metaId" text COLLATE pg_catalog."default",
    price text COLLATE pg_catalog."default",
    burned boolean,
    "transferCount" numeric,
    state text COLLATE pg_catalog."default",
    "storeId" text COLLATE pg_catalog."default" NOT NULL,
    "ownerId" text COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT tokens_pkey PRIMARY KEY (id)
)



ALTER TABLE public.tokens
    OWNER to mintbase;

-- Table: public.users

-- DROP TABLE public.users;

CREATE TABLE public.users
(
    id text COLLATE pg_catalog."default" NOT NULL,
    avatar text COLLATE pg_catalog."default",
    CONSTRAINT users_pkey PRIMARY KEY (id)
)



ALTER TABLE public.users
    OWNER to mintbase;