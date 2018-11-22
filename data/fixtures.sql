--
-- PostgreSQL database dump
--

-- Dumped from database version 9.6.10
-- Dumped by pg_dump version 10.5

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: plpgsql; Type: EXTENSION; Schema: -; Owner: 
--

CREATE EXTENSION IF NOT EXISTS plpgsql WITH SCHEMA pg_catalog;


--
-- Name: EXTENSION plpgsql; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION plpgsql IS 'PL/pgSQL procedural language';


--
-- Name: citext; Type: EXTENSION; Schema: -; Owner: 
--

CREATE EXTENSION IF NOT EXISTS citext WITH SCHEMA public;


--
-- Name: EXTENSION citext; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION citext IS 'data type for case-insensitive character strings';


--
-- Name: moddatetime; Type: EXTENSION; Schema: -; Owner: 
--

CREATE EXTENSION IF NOT EXISTS moddatetime WITH SCHEMA public;


--
-- Name: EXTENSION moddatetime; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION moddatetime IS 'functions for tracking last modification time';


--
-- Name: uuid-ossp; Type: EXTENSION; Schema: -; Owner: 
--

CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA public;


--
-- Name: EXTENSION "uuid-ossp"; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION "uuid-ossp" IS 'generate universally unique identifiers (UUIDs)';


SET default_tablespace = '';

SET default_with_oids = false;

--
-- Name: link_topics; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.link_topics (
    parent_id uuid NOT NULL,
    child_id uuid NOT NULL
);


ALTER TABLE public.link_topics OWNER TO postgres;

--
-- Name: links; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.links (
    organization_id uuid NOT NULL,
    id uuid DEFAULT public.uuid_generate_v1mc() NOT NULL,
    url text NOT NULL,
    title text DEFAULT ''::text NOT NULL,
    sha1 character varying(40) NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.links OWNER TO postgres;

--
-- Name: organizations; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.organizations (
    id uuid DEFAULT public.uuid_generate_v1mc() NOT NULL,
    name character varying(256) NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.organizations OWNER TO postgres;

--
-- Name: schema_migrations; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.schema_migrations (
    version bigint NOT NULL,
    dirty boolean NOT NULL
);


ALTER TABLE public.schema_migrations OWNER TO postgres;

--
-- Name: topic_topics; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.topic_topics (
    parent_id uuid NOT NULL,
    child_id uuid NOT NULL
);


ALTER TABLE public.topic_topics OWNER TO postgres;

--
-- Name: topics; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.topics (
    organization_id uuid NOT NULL,
    id uuid DEFAULT public.uuid_generate_v1mc() NOT NULL,
    name character varying(256) NOT NULL,
    description text,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.topics OWNER TO postgres;

--
-- Name: users; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.users (
    id uuid DEFAULT public.uuid_generate_v1mc() NOT NULL,
    name character varying(256) NOT NULL,
    primary_email public.citext NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL
);


ALTER TABLE public.users OWNER TO postgres;

--
-- Data for Name: link_topics; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.link_topics (parent_id, child_id) FROM stdin;
f1e1b1d6-ee02-11e8-987f-5f8b10f1ae1b	0d7fbb5a-ee07-11e8-8995-3b56ae45c0b3
f1e1b1d6-ee02-11e8-987f-5f8b10f1ae1b	1dbe632c-ee07-11e8-8995-77edd669ec46
492019e8-ee07-11e8-8642-239c847b42a7	4d141478-ee07-11e8-ae67-53c85dce4ba5
45dca814-e6f0-11e8-8bc1-b363da4aeace	4d141478-ee07-11e8-ae67-53c85dce4ba5
87240524-ee07-11e8-a945-5b5230ce136b	897061a6-ee07-11e8-a945-a32c887b43c4
c922b0ce-ee07-11e8-aa9d-131c55a142f2	caf89828-ee07-11e8-aa9d-a7c515f0a30c
45dcab0c-e6f0-11e8-8bc1-bbb431f062c7	10120416-eadf-11e8-8231-db1081f8c4fc
45dca814-e6f0-11e8-8bc1-b363da4aeace	10120416-eadf-11e8-8231-db1081f8c4fc
45dcaaf8-e6f0-11e8-8bc1-d7a04cdda708	fec9434a-eade-11e8-8231-3be3240b1542
45dcaad0-e6f0-11e8-8bc1-677f3b3c362f	fec9434a-eade-11e8-8231-3be3240b1542
45dcab0c-e6f0-11e8-8bc1-bbb431f062c7	fec9434a-eade-11e8-8231-3be3240b1542
07aa840c-ee08-11e8-ad4d-3b2ce62142ec	caf89828-ee07-11e8-aa9d-a7c515f0a30c
3c35074c-ee08-11e8-9465-338bc5df4123	3e6c6fbe-ee08-11e8-bb66-77b3e2753eca
9b022cf0-ee08-11e8-9465-5363c950bfbc	9dad3f1c-ee08-11e8-bb66-735b417eb99b
c922b0ce-ee07-11e8-aa9d-131c55a142f2	9dad3f1c-ee08-11e8-bb66-735b417eb99b
c922b0ce-ee07-11e8-aa9d-131c55a142f2	c4cda884-ee08-11e8-b1d2-db8198e90692
9b022cf0-ee08-11e8-9465-5363c950bfbc	c4cda884-ee08-11e8-b1d2-db8198e90692
45dca814-e6f0-11e8-8bc1-b363da4aeace	dde9cbd6-ee08-11e8-b1d2-572b8f794183
d9a643e4-ee0b-11e8-b1d2-639e5dd7d82d	dbdbe682-ee0b-11e8-b1d2-677e1f1f2f91
8df28630-ee0e-11e8-be24-cf8ad0267553	92ee44da-ee0e-11e8-be24-db545f10e1f9
5680a7a8-ee73-11e8-ba50-432dc68d001a	5b0b260e-ee73-11e8-ba50-5ffa77fcbe57
b7c14ab8-ee73-11e8-ba50-233c6ad09102	bc615504-ee73-11e8-ba50-2ffe13c3897c
1bed8c36-ee74-11e8-ba50-bb71aca3cf28	bc615504-ee73-11e8-ba50-2ffe13c3897c
6d8c25a2-ee74-11e8-a81c-f3fdddedf16f	bc615504-ee73-11e8-ba50-2ffe13c3897c
b7c14ab8-ee73-11e8-ba50-233c6ad09102	2ac60822-ee75-11e8-a81c-5ff309b5de84
4d4152ee-ee75-11e8-9c27-47cb09a2892a	2ac60822-ee75-11e8-a81c-5ff309b5de84
3c35074c-ee08-11e8-9465-338bc5df4123	95b6e976-ee75-11e8-9c27-03de8080687c
b7c14ab8-ee73-11e8-ba50-233c6ad09102	95b6e976-ee75-11e8-9c27-03de8080687c
3c35074c-ee08-11e8-9465-338bc5df4123	f871fe4c-ee76-11e8-947a-77b30d881a83
3c35074c-ee08-11e8-9465-338bc5df4123	0e7ae514-ee77-11e8-947a-77f1b1a0b3c8
b7c14ab8-ee73-11e8-ba50-233c6ad09102	1e60d664-ee77-11e8-b119-a7d48fac799c
b7c14ab8-ee73-11e8-ba50-233c6ad09102	25db63d0-ee79-11e8-b119-bf30e8234ce4
87240524-ee07-11e8-a945-5b5230ce136b	50cfc824-ee79-11e8-98f9-e3204b163fba
c922b0ce-ee07-11e8-aa9d-131c55a142f2	959d07be-ee79-11e8-ad4e-c32f3bd6fb1c
c922b0ce-ee07-11e8-aa9d-131c55a142f2	cc289d98-ee79-11e8-ad4e-4f1b31a97790
f1e1b1d6-ee02-11e8-987f-5f8b10f1ae1b	ea862bca-ee79-11e8-a9cb-afc3fd6990fe
3c35074c-ee08-11e8-9465-338bc5df4123	b3d9e700-ee7a-11e8-bb6f-17c10d49e67b
3c35074c-ee08-11e8-9465-338bc5df4123	35334102-ee7b-11e8-bb6f-2fed4f695d00
df63295e-ee02-11e8-9e36-17d56b662bc8	5e4cd742-ee7b-11e8-bb6f-e33b2e13652c
3c35074c-ee08-11e8-9465-338bc5df4123	7585ca9a-ee7b-11e8-b88e-83b7e9821fe1
3c35074c-ee08-11e8-9465-338bc5df4123	b0633868-ee7c-11e8-b88e-af9924a3aaef
9cdd6e02-eec3-11e8-9afc-1fa8f0db9ccf	a18d2000-eec3-11e8-9afc-138d16aeec63
dae01074-eec3-11e8-9afc-4be17860c00c	dce3a12e-eec3-11e8-bfbe-0f6be7d8bb14
2245933a-eec4-11e8-b546-a70043135c65	24f8a6f8-eec4-11e8-9c83-cf64a2f02829
2245933a-eec4-11e8-b546-a70043135c65	29fb0312-eec4-11e8-b546-ff4cb3cdb805
df63295e-ee02-11e8-9e36-17d56b662bc8	6977c746-eec4-11e8-b546-1b758c7e4e34
df63295e-ee02-11e8-9e36-17d56b662bc8	c5422792-eec4-11e8-b546-238f1404f38b
3c35074c-ee08-11e8-9465-338bc5df4123	ebfe3a74-eec4-11e8-8bc1-7f229ad8775a
3c35074c-ee08-11e8-9465-338bc5df4123	0fc5c936-eec5-11e8-8bc1-db5830baf62b
3c35074c-ee08-11e8-9465-338bc5df4123	1f3ba264-eec5-11e8-8bc1-0b72db06d3fe
3c35074c-ee08-11e8-9465-338bc5df4123	3288d788-eec5-11e8-8bc1-c3b3c5902d12
3c35074c-ee08-11e8-9465-338bc5df4123	4043f88a-eec5-11e8-baaa-e3dceb8d4fbf
d9a643e4-ee0b-11e8-b1d2-639e5dd7d82d	65ea947c-eec5-11e8-baaa-dfd390996a29
d9a643e4-ee0b-11e8-b1d2-639e5dd7d82d	433cb986-eec6-11e8-baaa-47d5d38f7548
\.


--
-- Data for Name: links; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.links (organization_id, id, url, title, sha1, created_at, updated_at) FROM stdin;
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	3e6c6fbe-ee08-11e8-bb66-77b3e2753eca	https://www.inkandswitch.com/slow-software.html	Slow Software	dc9339c15d9b66d243ac81f37cc527ed207f20f8	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	9dad3f1c-ee08-11e8-bb66-735b417eb99b	https://www.zdnet.com/article/popular-dark-web-hosting-provider-got-hacked-6500-sites-down/	Popular Dark Web hosting provider got hacked, 6,500 sites down | ZDNet	711a56a337981ae0de4929d8de1b9a9ff1534034	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	c4cda884-ee08-11e8-b1d2-db8198e90692	https://news.ycombinator.com/item?id=18504490	Popular Dark Web hosting provider hacked, 6,500 sites down | Hacker News	f24fd9588600f1ca5974061fcab97a38593c9f82	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	dde9cbd6-ee08-11e8-b1d2-572b8f794183	https://phys.org/news/2018-11-dog-cow-horse-pig-rabbit.html	The taming of the dog, cow, horse, pig and rabbit	040c320ad8cc0bdba35172df0c1316e123f880b2	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	dbdbe682-ee0b-11e8-b1d2-677e1f1f2f91	https://www.nytimes.com/2018/11/21/us/paradise-fires-emergency-alerts.html	A Frantic Call, a Neighbor’s Knock, but Few Official Alerts as Wildfire Closed In - The New York Times	cc3a42083f0094cc10f53aa49323f26472cc12a7	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	92ee44da-ee0e-11e8-be24-db545f10e1f9	https://www.reddit.com/r/AskReddit/comments/9z40sm/former_undercover_cops_of_reddit_what_is_the/	r/AskReddit - Former undercover cops of Reddit, what is the craziest thing you had to do to not blow your cover?	29e521205b6f1b421c35f49c95c774f6358e2578	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	5b0b260e-ee73-11e8-ba50-5ffa77fcbe57	https://gcemetery.co/	The Google Cemetery	58f88a6810dda73f0504935c5ca71349aa903bd2	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	bc615504-ee73-11e8-ba50-2ffe13c3897c	https://www.bbc.com/news/uk-wales-46294000	Treasure hunter finds buried chariot	d611bf8ea1acc580f7c86e11ef5a41ad12015ad6	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	2ac60822-ee75-11e8-a81c-5ff309b5de84	https://news.ycombinator.com/item?id=18509243	Pembrokeshire treasure hunter unearths Celtic chariot | Hacker News	75e9db9b511f7591c5a8beb0b14f5df14c51ee0f	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	95b6e976-ee75-11e8-9c27-03de8080687c	https://www.zeit.de/digital/games/2018-11/computer-games-gdr-stasi-surveillance-gamer-crowd/komplettansicht	Video Games In East Germany: The Stasi Played Along	6f35a5350f8a62ea8896095a4d37ee1cbf2d7153	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	fec9434a-eade-11e8-8231-3be3240b1542	https://github.com/	Github	d7b3438d97f335e612a566a731eea5acb8fe83c8	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	0a77ebea-ee01-11e8-86f0-5b6a2394f2e2	https://golang.org/pkg/log/	log - The Go Programming Language	51afa0ea2bd70aa9f40b2d0a65a2178786252a4e	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	f871fe4c-ee76-11e8-947a-77b30d881a83	https://amp.rs/	Amp - A complete text editor for your terminal	aa6081f1f941a408423da0e5ad660bc46120caf3	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	10120416-eadf-11e8-8231-db1081f8c4fc	https://www.google.com/	Google	595c3cce2409a55c13076f1bac5edee529fc2e58	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	0e7ae514-ee77-11e8-947a-77f1b1a0b3c8	https://news.ycombinator.com/item?id=18502196	Amp – A complete text editor for the terminal | Hacker News	9b21432aeaefa004e9e5853aee231cb20e7b5960	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	1e60d664-ee77-11e8-b119-a7d48fac799c	https://www.1843magazine.com/culture/look-closer/inside-the-court-of-ashurbanipal-king-of-the-world	Inside the court of Ashurbanipal, king of the world	3b30634daed117864302a360d060801d5ad69c1b	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	25db63d0-ee79-11e8-b119-bf30e8234ce4	https://news.ycombinator.com/item?id=18508530	Inside the court of Ashurbanipal, king of the world | Hacker News	46131418147a3bb996d582d5b240543c938532b7	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	0d7fbb5a-ee07-11e8-8995-3b56ae45c0b3	https://news.ycombinator.com/item?id=18504300	My hiring experience as a submarine sonar operator in the Norwegian Navy | Hacker News	55182389cc4f963bc6e8e1821689d8e5d5b82a78	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	1dbe632c-ee07-11e8-8995-77edd669ec46	https://www.brautaset.org/articles/2018/submarine-sonar-hiring.html	My hiring experience as a submarine sonar operator in the Norwegian Navy	6b12407e657f1bf5cef7782c4300232a82a58b8f	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	4d141478-ee07-11e8-ae67-53c85dce4ba5	https://www.scientificamerican.com/article/silent-and-simple-ion-engine-powers-a-plane-with-no-moving-parts/	Silent and Simple Ion Engine Powers a Plane with No Moving Parts	e38e13d1153527faccae187642938fe47bf0a4c3	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	897061a6-ee07-11e8-a945-a32c887b43c4	https://www.cia.gov/library/center-for-the-study-of-intelligence/csi-publications/books-and-monographs/psychology-of-intelligence-analysis/art4.html	Chapter 1  — Central Intelligence Agency	777771953559d7af66cf4e4c2d213515a2658be6	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	caf89828-ee07-11e8-aa9d-a7c515f0a30c	https://techcrunch.com/2018/11/21/amazon-admits-it-exposed-customer-email-addresses-doubles-down-on-secrecy/	Amazon admits it exposed customer email addresses, but refuses to give details	dadd4fd52333e89fa9c6c5db9f435287b3ce9652	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	50cfc824-ee79-11e8-98f9-e3204b163fba	https://news.ycombinator.com/item?id=18497985	Gabor Maté on Addiction to Ideology and Social Media [video] | Hacker News	fc69c61c8890fd13d64fc73936a3a443b47bf80c	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	959d07be-ee79-11e8-ad4e-c32f3bd6fb1c	https://blog.georgovassilis.com/2016/04/16/advanced-web-security-topics/	Advanced web security topics	afc550885532a243736cb091da38a6f61355b3da	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	cc289d98-ee79-11e8-ad4e-4f1b31a97790	https://news.ycombinator.com/item?id=18508211	Advanced web security topics | Hacker News	50261f60471ffaa98f31a098faa4bd5c548141dc	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	ea862bca-ee79-11e8-a9cb-afc3fd6990fe	https://news.ycombinator.com/item?id=18507407	Ask HN: Why not more hiring of junior devs, then on-the-job-training? | Hacker News	46d0d0d20883d2b5b17ef02daa0deecfea379e0d	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	b3d9e700-ee7a-11e8-bb6f-17c10d49e67b	https://dba.stackexchange.com/questions/107475/how-to-best-store-a-timestamp-in-postgresql	datatypes - How to best store a timestamp in PostgreSQL? - Database Administrators Stack Exchange	c76e1c494f1e64802b06c4be93279d3d14fe9ae8	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	35334102-ee7b-11e8-bb6f-2fed4f695d00	https://stackoverflow.com/questions/28872761/time-zone-storage-in-data-type-timestamp-with-time-zone/28876266#28876266	sql - Time zone storage in data type "timestamp with time zone" - Stack Overflow	d5aebe71c77c9f1b658c1f14c19c24d87e0987ea	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	5e4cd742-ee7b-11e8-bb6f-e33b2e13652c	https://en.wikipedia.org/wiki/Daylight_saving_time	Daylight saving time - Wikipedia	67b179c6f5a078f37513a869f979cf72625d65a8	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	7585ca9a-ee7b-11e8-b88e-83b7e9821fe1	https://stackoverflow.com/questions/9571392/ignoring-time-zones-altogether-in-rails-and-postgresql/9576170	Ignoring time zones altogether in Rails and PostgreSQL - Stack Overflow	6bed84cd98fadd2022fc76ac7c6357770830103a	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	b0633868-ee7c-11e8-b88e-af9924a3aaef	https://dba.stackexchange.com/questions/156980/create-a-trigger-on-all-the-last-modified-columns-in-postgresql	Create a trigger on all the last_modified columns in PostgreSQL - Database Administrators Stack Exchange	e5b854428d7160a42c596794fb9e4dd7a8e3a5b9	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	a18d2000-eec3-11e8-9afc-138d16aeec63	https://www.chemistryworld.com/news/world-first-as-wind-turbine-upgraded-with-high-temperature-superconductor/3009780.article	World first as wind turbine upgraded with high temperature superconductor	096f75a6bc627d87a4c89c8d46726ba2c6837613	2018-11-22 20:01:03.120373-06	2018-11-22 20:01:03.120373-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	dce3a12e-eec3-11e8-bfbe-0f6be7d8bb14	https://www.nytimes.com/interactive/2018/11/21/nyregion/new-york-storefronts-mystery-font.html	The Mystery Font That Took Over New York - The New York Times	2bfaaae9baf924fd6e9f3bbf9158978183d0ae08	2018-11-22 20:02:42.672408-06	2018-11-22 20:02:42.672408-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	24f8a6f8-eec4-11e8-9c83-cf64a2f02829	http://bradfrost.com/blog/post/ditching-the-macbook-pro-for-a-macbook-air/	Ditching the MacBook Pro for a MacBook Air	f476ddad9a5901f42d9119a975c05a510f0e4cf4	2018-11-22 20:04:43.605547-06	2018-11-22 20:04:43.605547-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	29fb0312-eec4-11e8-b546-ff4cb3cdb805	https://news.ycombinator.com/item?id=18511184	Ditching the MacBook Pro for a MacBook Air | Hacker News	04308c9ff1d9c2c8ec914dbd91320b90dc05e2b5	2018-11-22 20:04:52.01121-06	2018-11-22 20:04:52.01121-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	6977c746-eec4-11e8-b546-1b758c7e4e34	https://petapixel.com/2018/11/20/the-story-behind-that-ikea-photo-of-amsterdam/	The Story Behind That IKEA Photo of Amsterdam	b185c53b2e56e4ab195396927602605caaa0f6e6	2018-11-22 20:06:38.525254-06	2018-11-22 20:06:38.525254-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	c5422792-eec4-11e8-b546-238f1404f38b	https://news.ycombinator.com/item?id=18512268	The Story Behind the IKEA Photo of Amsterdam | Hacker News	471a194d3441e7e0e5e033ec5340824efd3f4a02	2018-11-22 20:09:12.524196-06	2018-11-22 20:09:12.524196-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	ebfe3a74-eec4-11e8-8bc1-7f229ad8775a	https://gitless.com/	Gitless	cc1d25957da9ccfc58dfdb77df2c3dce67ad448a	2018-11-22 20:10:17.508657-06	2018-11-22 20:10:17.508657-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	0fc5c936-eec5-11e8-8bc1-db5830baf62b	https://github.com/developit/htm	developit/htm	d9411e0f1660fbf9cb5288f8321839e36881ff9d	2018-11-22 20:11:17.538203-06	2018-11-22 20:11:17.538203-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	1f3ba264-eec5-11e8-8bc1-0b72db06d3fe	https://news.ycombinator.com/item?id=18510922	Hyperscript Tagged Markup: JSX alternative using standard tagged templates | Hacker News	fe4fc9777cd180d83340c76ac0198ac2f8e8ac43	2018-11-22 20:11:43.476535-06	2018-11-22 20:11:43.476535-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	3288d788-eec5-11e8-8bc1-c3b3c5902d12	https://news.ycombinator.com/item?id=18512294	Goboy: Multi-Platform Nintendo Game Boy Color Emulator Written in Go | Hacker News	8fb72801e6485c9498d5037de9f22bcc08d2534b	2018-11-22 20:12:15.859163-06	2018-11-22 20:12:15.859163-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	4043f88a-eec5-11e8-baaa-e3dceb8d4fbf	https://news.ycombinator.com/item?id=18512304	Thank you to dang and sctb | Hacker News	07405b685c4b0d2812ee39a5cd0e5f8b6f02cb7a	2018-11-22 20:12:38.894652-06	2018-11-22 20:12:38.894652-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	65ea947c-eec5-11e8-baaa-dfd390996a29	https://www.nytimes.com/2018/11/11/us/california-fire-paradise.html	Forced Out by Deadly Fires, Then Trapped in Traffic - The New York Times	cbd65f69dc5ab576f8d108992c265fe86ea06f54	2018-11-22 20:13:42.063638-06	2018-11-22 20:13:42.063638-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	433cb986-eec6-11e8-baaa-47d5d38f7548	https://news.ycombinator.com/item?id=18511334	Forced Out by Deadly Fires, Then Trapped in Traffic | Hacker News	d5cb0af8101cb55fcc3f0ad72df61741ff874262	2018-11-22 20:19:53.378396-06	2018-11-22 20:19:53.378396-06
\.


--
-- Data for Name: organizations; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.organizations (id, name, created_at, updated_at) FROM stdin;
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	Tyrell Corporation	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
\.


--
-- Data for Name: schema_migrations; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.schema_migrations (version, dirty) FROM stdin;
1542907673	f
\.


--
-- Data for Name: topic_topics; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.topic_topics (parent_id, child_id) FROM stdin;
45dca814-e6f0-11e8-8bc1-b363da4aeace	45dcaad0-e6f0-11e8-8bc1-677f3b3c362f
45dca814-e6f0-11e8-8bc1-b363da4aeace	45dcaaf8-e6f0-11e8-8bc1-d7a04cdda708
45dca814-e6f0-11e8-8bc1-b363da4aeace	45dcab0c-e6f0-11e8-8bc1-bbb431f062c7
df63295e-ee02-11e8-9e36-17d56b662bc8	f1e1b1d6-ee02-11e8-987f-5f8b10f1ae1b
df63295e-ee02-11e8-9e36-17d56b662bc8	492019e8-ee07-11e8-8642-239c847b42a7
45dcaad0-e6f0-11e8-8bc1-677f3b3c362f	45644e98-ecd9-11e8-8e0e-6fa75df8779e
45dca814-e6f0-11e8-8bc1-b363da4aeace	87240524-ee07-11e8-a945-5b5230ce136b
df63295e-ee02-11e8-9e36-17d56b662bc8	c922b0ce-ee07-11e8-aa9d-131c55a142f2
df63295e-ee02-11e8-9e36-17d56b662bc8	f68eb472-ee07-11e8-aa9d-23d8a9db2265
f68eb472-ee07-11e8-aa9d-23d8a9db2265	07aa840c-ee08-11e8-ad4d-3b2ce62142ec
df63295e-ee02-11e8-9e36-17d56b662bc8	3c35074c-ee08-11e8-9465-338bc5df4123
df63295e-ee02-11e8-9e36-17d56b662bc8	9b022cf0-ee08-11e8-9465-5363c950bfbc
df63295e-ee02-11e8-9e36-17d56b662bc8	d9a643e4-ee0b-11e8-b1d2-639e5dd7d82d
df63295e-ee02-11e8-9e36-17d56b662bc8	8df28630-ee0e-11e8-be24-cf8ad0267553
f68eb472-ee07-11e8-aa9d-23d8a9db2265	5680a7a8-ee73-11e8-ba50-432dc68d001a
df63295e-ee02-11e8-9e36-17d56b662bc8	b7c14ab8-ee73-11e8-ba50-233c6ad09102
df63295e-ee02-11e8-9e36-17d56b662bc8	1278c44a-ee74-11e8-b558-47a668d1102a
1278c44a-ee74-11e8-b558-47a668d1102a	1bed8c36-ee74-11e8-ba50-bb71aca3cf28
b7c14ab8-ee73-11e8-ba50-233c6ad09102	6d8c25a2-ee74-11e8-a81c-f3fdddedf16f
df63295e-ee02-11e8-9e36-17d56b662bc8	48dd73b8-ee75-11e8-a81c-6bfd74718954
48dd73b8-ee75-11e8-a81c-6bfd74718954	4d4152ee-ee75-11e8-9c27-47cb09a2892a
45dca814-e6f0-11e8-8bc1-b363da4aeace	9cdd6e02-eec3-11e8-9afc-1fa8f0db9ccf
df63295e-ee02-11e8-9e36-17d56b662bc8	dae01074-eec3-11e8-9afc-4be17860c00c
df63295e-ee02-11e8-9e36-17d56b662bc8	2245933a-eec4-11e8-b546-a70043135c65
\.


--
-- Data for Name: topics; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.topics (organization_id, id, name, description, created_at, updated_at) FROM stdin;
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	45dcaaf8-e6f0-11e8-8bc1-d7a04cdda708	Chemistry	\N	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	df63295e-ee02-11e8-9e36-17d56b662bc8	Everything	\N	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	f1e1b1d6-ee02-11e8-987f-5f8b10f1ae1b	Hiring for a business	\N	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	492019e8-ee07-11e8-8642-239c847b42a7	Engineering	\N	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	87240524-ee07-11e8-a945-5b5230ce136b	Psychology	\N	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	c922b0ce-ee07-11e8-aa9d-131c55a142f2	Computer security	\N	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	f68eb472-ee07-11e8-aa9d-23d8a9db2265	Organizations, businesses and trusts	\N	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	07aa840c-ee08-11e8-ad4d-3b2ce62142ec	Amazon.com, Inc	\N	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	3c35074c-ee08-11e8-9465-338bc5df4123	Software	\N	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	9b022cf0-ee08-11e8-9465-5363c950bfbc	The dark web	\N	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	d9a643e4-ee0b-11e8-b1d2-639e5dd7d82d	Natural disasters	\N	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	8df28630-ee0e-11e8-be24-cf8ad0267553	Crime and law enforcement	\N	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	5680a7a8-ee73-11e8-ba50-432dc68d001a	Google LLC	\N	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	b7c14ab8-ee73-11e8-ba50-233c6ad09102	History	\N	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	1278c44a-ee74-11e8-b558-47a668d1102a	Places	\N	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	1bed8c36-ee74-11e8-ba50-bb71aca3cf28	Wales	\N	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	45dca814-e6f0-11e8-8bc1-b363da4aeace	Science		2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	6d8c25a2-ee74-11e8-a81c-f3fdddedf16f	The Iron Age	\N	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	48dd73b8-ee75-11e8-a81c-6bfd74718954	Legislation	\N	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	4d4152ee-ee75-11e8-9c27-47cb09a2892a	The Treasure Act 1996 (UK)	\N	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	45dcab0c-e6f0-11e8-8bc1-bbb431f062c7	Physics		2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	45644e98-ecd9-11e8-8e0e-6fa75df8779e	Zoology	\N	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	45dcaad0-e6f0-11e8-8bc1-677f3b3c362f	Biology		2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	9cdd6e02-eec3-11e8-9afc-1fa8f0db9ccf	Superconductors	\N	2018-11-22 20:00:55.257861-06	2018-11-22 20:00:55.257861-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	dae01074-eec3-11e8-9afc-4be17860c00c	Design	\N	2018-11-22 20:02:39.29478-06	2018-11-22 20:02:39.29478-06
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	2245933a-eec4-11e8-b546-a70043135c65	Computers	\N	2018-11-22 20:04:39.077148-06	2018-11-22 20:04:39.077148-06
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.users (id, name, primary_email, created_at, updated_at) FROM stdin;
45dc948c-e6f0-11e8-8bc1-97e5a947cde4	Gnusto	gnusto@tyrell.test	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dca10c-e6f0-11e8-8bc1-0bba87ab695e	Frotz	frotz@tyrell.test	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dca1ac-e6f0-11e8-8bc1-7f2417a740c0	Yomin	yomin@tyrell.test	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dca1fc-e6f0-11e8-8bc1-9f412923ac8b	Bozbar	bozbar@tyrell.test	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
45dca260-e6f0-11e8-8bc1-a7bab2abca4f	Rezrov	rezrov@tyrell.test	2018-11-22 11:52:19.627001-06	2018-11-22 11:52:19.627001-06
\.


--
-- Name: link_topics link_topics_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.link_topics
    ADD CONSTRAINT link_topics_pkey PRIMARY KEY (parent_id, child_id);


--
-- Name: links links_organization_sha1_idx; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.links
    ADD CONSTRAINT links_organization_sha1_idx UNIQUE (organization_id, sha1);


--
-- Name: links links_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.links
    ADD CONSTRAINT links_pkey PRIMARY KEY (id);


--
-- Name: organizations organizations_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.organizations
    ADD CONSTRAINT organizations_pkey PRIMARY KEY (id);


--
-- Name: schema_migrations schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.schema_migrations
    ADD CONSTRAINT schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: topics topics_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.topics
    ADD CONSTRAINT topics_pkey PRIMARY KEY (id);


--
-- Name: topic_topics topics_topics_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.topic_topics
    ADD CONSTRAINT topics_topics_pkey PRIMARY KEY (parent_id, child_id);


--
-- Name: users users_email_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_email_key UNIQUE (primary_email);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: topics_links_child_parent_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX topics_links_child_parent_idx ON public.link_topics USING btree (child_id, parent_id);


--
-- Name: topics_topics_child_parent_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX topics_topics_child_parent_idx ON public.topic_topics USING btree (child_id, parent_id);


--
-- Name: links links_updated_at; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER links_updated_at BEFORE UPDATE ON public.links FOR EACH ROW EXECUTE PROCEDURE public.moddatetime('updated_at');


--
-- Name: organizations organizations_updated_at; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER organizations_updated_at BEFORE UPDATE ON public.organizations FOR EACH ROW EXECUTE PROCEDURE public.moddatetime('updated_at');


--
-- Name: topics topics_updated_at; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER topics_updated_at BEFORE UPDATE ON public.topics FOR EACH ROW EXECUTE PROCEDURE public.moddatetime('updated_at');


--
-- Name: users users_updated_at; Type: TRIGGER; Schema: public; Owner: postgres
--

CREATE TRIGGER users_updated_at BEFORE UPDATE ON public.users FOR EACH ROW EXECUTE PROCEDURE public.moddatetime('updated_at');


--
-- Name: link_topics link_topics_child_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.link_topics
    ADD CONSTRAINT link_topics_child_id_fkey FOREIGN KEY (child_id) REFERENCES public.links(id) ON DELETE CASCADE;


--
-- Name: link_topics link_topics_parent_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.link_topics
    ADD CONSTRAINT link_topics_parent_id_fkey FOREIGN KEY (parent_id) REFERENCES public.topics(id) ON DELETE CASCADE;


--
-- Name: links links_organization_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.links
    ADD CONSTRAINT links_organization_id_fkey FOREIGN KEY (organization_id) REFERENCES public.organizations(id) ON DELETE CASCADE;


--
-- Name: topic_topics topic_topics_child_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.topic_topics
    ADD CONSTRAINT topic_topics_child_id_fkey FOREIGN KEY (child_id) REFERENCES public.topics(id) ON DELETE CASCADE;


--
-- Name: topics topics_organization_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.topics
    ADD CONSTRAINT topics_organization_id_fkey FOREIGN KEY (organization_id) REFERENCES public.organizations(id) ON DELETE CASCADE;


--
-- Name: topic_topics topics_topics_parent_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.topic_topics
    ADD CONSTRAINT topics_topics_parent_id_fkey FOREIGN KEY (parent_id) REFERENCES public.topics(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

