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

--
-- TOC entry 202 (class 1259 OID 16822)
-- Name: stores; Type: TABLE; Schema: public; Owner: flux
--

CREATE TABLE public.stores (
    id text,
    name text,
    symbol text,
    "totalSupply" numeric,
    "tokenCount" numeric,
    "boughtCount" numeric,
    "transferCount" numeric,
    owner text
);


ALTER TABLE public.stores OWNER TO flux;

--
-- TOC entry 204 (class 1259 OID 16834)
-- Name: things; Type: TABLE; Schema: public; Owner: flux
--

CREATE TABLE public.things (
    id text,
    minter text,
    "timestamp" timestamp without time zone,
    burned boolean,
    "forSale" boolean,
    "metaId" text
);


ALTER TABLE public.things OWNER TO flux;

--
-- TOC entry 205 (class 1259 OID 16840)
-- Name: tokens; Type: TABLE; Schema: public; Owner: flux
--

CREATE TABLE public.tokens (
    id text,
    "tokenId" text,
    "metaId" text,
    price text,
    burned boolean,
    "transferCount" numeric
);


ALTER TABLE public.tokens OWNER TO flux;

--
-- TOC entry 203 (class 1259 OID 16828)
-- Name: users; Type: TABLE; Schema: public; Owner: flux
--

CREATE TABLE public.users (
    id text,
    avatar text
);


ALTER TABLE public.users OWNER TO flux;

-- Completed on 2020-09-16 13:54:01 CEST

--
-- PostgreSQL database dump complete
--

