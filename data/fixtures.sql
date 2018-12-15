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
-- Name: pgcrypto; Type: EXTENSION; Schema: -; Owner: 
--

CREATE EXTENSION IF NOT EXISTS pgcrypto WITH SCHEMA public;


--
-- Name: EXTENSION pgcrypto; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION pgcrypto IS 'cryptographic functions';


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
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    repository_id uuid NOT NULL
);


ALTER TABLE public.links OWNER TO postgres;

--
-- Name: organization_members; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.organization_members (
    organization_id uuid NOT NULL,
    user_id uuid NOT NULL,
    owner boolean DEFAULT false NOT NULL
);


ALTER TABLE public.organization_members OWNER TO postgres;

--
-- Name: organizations; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.organizations (
    id uuid DEFAULT public.uuid_generate_v1mc() NOT NULL,
    name character varying(256) NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    login character varying(256) NOT NULL,
    description character varying(256),
    public boolean DEFAULT false NOT NULL,
    system boolean DEFAULT false NOT NULL
);


ALTER TABLE public.organizations OWNER TO postgres;

--
-- Name: repositories; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.repositories (
    id uuid DEFAULT public.uuid_generate_v1mc() NOT NULL,
    organization_id uuid NOT NULL,
    name character varying(256) NOT NULL,
    owner_id uuid NOT NULL,
    system boolean DEFAULT false NOT NULL
);


ALTER TABLE public.repositories OWNER TO postgres;

--
-- Name: schema_migrations; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.schema_migrations (
    version bigint NOT NULL,
    dirty boolean NOT NULL
);


ALTER TABLE public.schema_migrations OWNER TO postgres;

--
-- Name: sessions; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.sessions (
    id integer NOT NULL,
    session_id bytea DEFAULT public.digest((random())::text, 'sha256'::text) NOT NULL,
    user_id uuid NOT NULL
);


ALTER TABLE public.sessions OWNER TO postgres;

--
-- Name: sessions_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.sessions_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.sessions_id_seq OWNER TO postgres;

--
-- Name: sessions_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.sessions_id_seq OWNED BY public.sessions.id;


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
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    repository_id uuid NOT NULL,
    root boolean DEFAULT false NOT NULL
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
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    github_username character varying(256),
    github_avatar_url character varying(1000),
    login character varying(256) NOT NULL
);


ALTER TABLE public.users OWNER TO postgres;

--
-- Name: sessions id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.sessions ALTER COLUMN id SET DEFAULT nextval('public.sessions_id_seq'::regclass);


--
-- Data for Name: link_topics; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.link_topics (parent_id, child_id) FROM stdin;
f1e1b1d6-ee02-11e8-987f-5f8b10f1ae1b	0d7fbb5a-ee07-11e8-8995-3b56ae45c0b3
f1e1b1d6-ee02-11e8-987f-5f8b10f1ae1b	1dbe632c-ee07-11e8-8995-77edd669ec46
87240524-ee07-11e8-a945-5b5230ce136b	897061a6-ee07-11e8-a945-a32c887b43c4
c922b0ce-ee07-11e8-aa9d-131c55a142f2	caf89828-ee07-11e8-aa9d-a7c515f0a30c
45dcab0c-e6f0-11e8-8bc1-bbb431f062c7	10120416-eadf-11e8-8231-db1081f8c4fc
45dca814-e6f0-11e8-8bc1-b363da4aeace	10120416-eadf-11e8-8231-db1081f8c4fc
bacbdf66-efa9-11e8-855e-2b4716af8be5	8ccbece0-f022-11e8-ba54-a701cebc41cb
07aa840c-ee08-11e8-ad4d-3b2ce62142ec	caf89828-ee07-11e8-aa9d-a7c515f0a30c
3c35074c-ee08-11e8-9465-338bc5df4123	3e6c6fbe-ee08-11e8-bb66-77b3e2753eca
9b022cf0-ee08-11e8-9465-5363c950bfbc	9dad3f1c-ee08-11e8-bb66-735b417eb99b
c922b0ce-ee07-11e8-aa9d-131c55a142f2	9dad3f1c-ee08-11e8-bb66-735b417eb99b
c922b0ce-ee07-11e8-aa9d-131c55a142f2	c4cda884-ee08-11e8-b1d2-db8198e90692
9b022cf0-ee08-11e8-9465-5363c950bfbc	c4cda884-ee08-11e8-b1d2-db8198e90692
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
789fc094-eec8-11e8-9ff0-8f787ff5cec5	bc1160f6-ef97-11e8-aac0-272abf43dea2
e634be48-eec8-11e8-bbb3-bf16b851a67e	6599de8c-ef98-11e8-99f9-779097e0bf76
b4863926-eec8-11e8-93f6-e336cdf75beb	a9f05818-ef98-11e8-99f9-df8afcb9b764
e634be48-eec8-11e8-bbb3-bf16b851a67e	b6d23b5a-ef98-11e8-bf0b-6f2e1f977c5e
3c35074c-ee08-11e8-9465-338bc5df4123	f8396834-ef98-11e8-9ace-f30ff229ccb6
3c35074c-ee08-11e8-9465-338bc5df4123	1f077e1a-ef99-11e8-9ace-173fec9de1b8
b4863926-eec8-11e8-93f6-e336cdf75beb	5e94fd82-ef99-11e8-9ace-27e171b810a9
b7c14ab8-ee73-11e8-ba50-233c6ad09102	bf09b950-ef99-11e8-94cf-3f0981918655
dae01074-eec3-11e8-9afc-4be17860c00c	dcaa107c-ef99-11e8-9ace-cb69e62e1dae
c7d122e8-ef9a-11e8-9041-9331578a5d68	ca09f7a6-ef9a-11e8-a161-dfc86e1e0da6
dae01074-eec3-11e8-9afc-4be17860c00c	d601d010-ef9a-11e8-a161-6bfabe60d300
b7c14ab8-ee73-11e8-ba50-233c6ad09102	0983ddf2-ef9b-11e8-8d14-5baf275349e1
b4863926-eec8-11e8-93f6-e336cdf75beb	45f21cfe-ef9b-11e8-8d14-2ba1f305205e
dae01074-eec3-11e8-9afc-4be17860c00c	52d97048-ef9b-11e8-886c-47ab437d4d4d
3c35074c-ee08-11e8-9465-338bc5df4123	b5ce0a74-ef9b-11e8-8d14-67ba1c14dcef
3c35074c-ee08-11e8-9465-338bc5df4123	bd115cf0-ef9b-11e8-8d14-c3e70a67da5f
1278c44a-ee74-11e8-b558-47a668d1102a	d90e2cb2-ef9b-11e8-b219-0702492ff807
3c35074c-ee08-11e8-9465-338bc5df4123	aae24886-ef9c-11e8-9826-1b3bb08fdbbd
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
d0a36ac6-eec7-11e8-9b89-1b0756390caf	d495b904-eec7-11e8-9b89-cfa68a7d207c
33b18df0-eec8-11e8-b9e7-270ae3464cf5	36c2b64a-eec8-11e8-9d24-9713d9c75592
789fc094-eec8-11e8-9ff0-8f787ff5cec5	7b077bce-eec8-11e8-9ff0-2b69edcddd6b
b7c14ab8-ee73-11e8-ba50-233c6ad09102	9526e0f8-eec8-11e8-849a-c34e40cc2227
b4863926-eec8-11e8-93f6-e336cdf75beb	b6f1e908-eec8-11e8-93f6-7fcf36c56f5f
e634be48-eec8-11e8-bbb3-bf16b851a67e	e9b26516-eec8-11e8-bbb3-8bd13fdf2fff
e634be48-eec8-11e8-bbb3-bf16b851a67e	5ce449a0-eec9-11e8-bbb3-9f3df9230ba8
e634be48-eec8-11e8-bbb3-bf16b851a67e	cbd1adc6-eec9-11e8-bbb3-db2b8881d52e
33b18df0-eec8-11e8-b9e7-270ae3464cf5	8e94ca2c-eecb-11e8-bbb3-075606f0461a
dae01074-eec3-11e8-9afc-4be17860c00c	edb27e1e-eecb-11e8-b93f-032a1d1e8730
33b18df0-eec8-11e8-b9e7-270ae3464cf5	fab0e8fc-eecc-11e8-bbb3-170f613af621
3c35074c-ee08-11e8-9465-338bc5df4123	693c13d6-eece-11e8-b7a6-eb18e45495e0
d0a36ac6-eec7-11e8-9b89-1b0756390caf	8a77d3f8-eed0-11e8-b0bb-afd686b6376e
b4863926-eec8-11e8-93f6-e336cdf75beb	98db95a6-eed0-11e8-bfb2-f7044d070ccf
33b18df0-eec8-11e8-b9e7-270ae3464cf5	2436f5dc-eed1-11e8-b0bb-f78739f8edef
df63295e-ee02-11e8-9e36-17d56b662bc8	6249da0a-ef4f-11e8-ad6b-076386c675bd
b7c14ab8-ee73-11e8-ba50-233c6ad09102	3ea8d98a-eed6-11e8-8de7-8f1ea67e5d8b
db093354-ef2d-11e8-a97b-73264e4fd2bf	dfb3b118-ef2d-11e8-a97b-c3b43a9a195b
c922b0ce-ee07-11e8-aa9d-131c55a142f2	08c60046-ef43-11e8-b802-4bc00ef1d22a
c922b0ce-ee07-11e8-aa9d-131c55a142f2	56efb816-ef43-11e8-b802-6718cf34ed85
8c616b16-ef48-11e8-af6a-93e617bb4c16	8f7bc29c-ef48-11e8-af6a-4f8876bdaa9c
f68eb472-ee07-11e8-aa9d-23d8a9db2265	92c8dcd6-ef4e-11e8-a49e-2fff1f9fdf1e
2245933a-eec4-11e8-b546-a70043135c65	7da9f70e-ef58-11e8-97a8-f3303474aa70
2245933a-eec4-11e8-b546-a70043135c65	d35a7362-ef5e-11e8-97a8-8f6e20dbc7c6
3c35074c-ee08-11e8-9465-338bc5df4123	32f0914c-ef61-11e8-add3-bfa4399ab606
3c35074c-ee08-11e8-9465-338bc5df4123	ae331398-ef61-11e8-add3-b39b7ea7844d
b9ce6270-ef66-11e8-994e-7b6205385b8c	c0077460-ef66-11e8-994e-d7feffa99f29
b9ce6270-ef66-11e8-994e-7b6205385b8c	c5f9835e-ef66-11e8-994e-23d967db8097
f68eb472-ee07-11e8-aa9d-23d8a9db2265	08791956-ef67-11e8-994e-ff881bf0d97e
8c616b16-ef48-11e8-af6a-93e617bb4c16	3d4abca6-ef68-11e8-aa37-2b21c89bd76a
8c616b16-ef48-11e8-af6a-93e617bb4c16	f44708ba-ef68-11e8-a1af-17b3db2b15ad
45dca814-e6f0-11e8-8bc1-b363da4aeace	2327d332-efa3-11e8-8b9a-7b6e3edd5934
3c35074c-ee08-11e8-9465-338bc5df4123	d0f9df9c-ef70-11e8-9894-4771acb08d56
3c35074c-ee08-11e8-9465-338bc5df4123	be804548-ef72-11e8-9894-a3d8d275a791
3c35074c-ee08-11e8-9465-338bc5df4123	41a0f0f8-ef73-11e8-a1b2-438674b0b6a4
3c35074c-ee08-11e8-9465-338bc5df4123	5e76f31c-ef73-11e8-a1b2-6bf2acf69702
3c35074c-ee08-11e8-9465-338bc5df4123	a39f0ee8-ef73-11e8-9894-2bcad5930372
876919a2-ef74-11e8-9bb5-8ba353cd1eac	8a15ea4a-ef74-11e8-8aa2-6b66d2d9bdc3
3e5fe9a2-ef74-11e8-9894-670527823de2	41166536-ef74-11e8-9894-8b9f0d759075
876919a2-ef74-11e8-9bb5-8ba353cd1eac	41166536-ef74-11e8-9894-8b9f0d759075
492019e8-ee07-11e8-8642-239c847b42a7	4d141478-ee07-11e8-ae67-53c85dce4ba5
45dcaaf8-e6f0-11e8-8bc1-d7a04cdda708	77b68630-ff68-11e8-befa-c7cdcb0356a1
99921f80-f022-11e8-8d18-a7147a02c2bc	9bdd8356-f022-11e8-8d18-a351a2cbaf74
ef6d0996-efa1-11e8-9c47-2ba4278db941	783e5e46-efa2-11e8-9c47-fbb892f95289
d0a36ac6-eec7-11e8-9b89-1b0756390caf	25abbdf8-ef76-11e8-89a9-4bfd0a8e3d1e
ce40a0c0-ef9c-11e8-9826-dbc543f3126a	d17b9844-ef9c-11e8-9826-e79b3460119d
1278c44a-ee74-11e8-b558-47a668d1102a	f162e5a4-ef9c-11e8-adea-33d098c6ae27
ce40a0c0-ef9c-11e8-9826-dbc543f3126a	021f5a8a-ef9d-11e8-adea-87ab9e69bacf
dae01074-eec3-11e8-9afc-4be17860c00c	1a960a32-ef9d-11e8-8487-ebf308f47aaf
1bed8c36-ee74-11e8-ba50-bb71aca3cf28	1a960a32-ef9d-11e8-8487-ebf308f47aaf
ce40a0c0-ef9c-11e8-9826-dbc543f3126a	67e9f2e8-ef9e-11e8-ac40-7341c4fc3aaf
3c35074c-ee08-11e8-9465-338bc5df4123	8b224972-ef9e-11e8-843e-13ebe9173a32
3c35074c-ee08-11e8-9465-338bc5df4123	be08d798-ef9e-11e8-843e-b35f24bdee27
b4863926-eec8-11e8-93f6-e336cdf75beb	bcfc7caa-ef9f-11e8-9d26-7fb83e5bfa96
45644e98-ecd9-11e8-8e0e-6fa75df8779e	f171992a-ef9f-11e8-9d26-7f3d46f17c61
b9ce6270-ef66-11e8-994e-7b6205385b8c	29dac70a-efa0-11e8-9d26-67a7dfdfebcc
ce40a0c0-ef9c-11e8-9826-dbc543f3126a	709b13de-efa0-11e8-9c47-fb7bab4fce38
b1ae5a48-efa0-11e8-9c47-57dd8ba1d211	b42c6620-efa0-11e8-973a-c3ca26fd0256
789fc094-eec8-11e8-9ff0-8f787ff5cec5	a839c370-efa1-11e8-9c47-c7961281ba96
ef6d0996-efa1-11e8-9c47-2ba4278db941	f2326d7e-efa1-11e8-973a-0b18966add17
ef6d0996-efa1-11e8-9c47-2ba4278db941	4724d7f6-efa0-11e8-9c47-53f873b5c385
ef6d0996-efa1-11e8-9c47-2ba4278db941	576bd562-ef9e-11e8-843e-7b1b11140f00
45644e98-ecd9-11e8-8e0e-6fa75df8779e	3a680f7c-efa2-11e8-9c47-c74f5bf75acf
b9ce6270-ef66-11e8-994e-7b6205385b8c	534cb1f0-efa2-11e8-973a-bb57dbf1dc21
bd9789e0-efa2-11e8-9c47-3b8ea26f89ae	c02485f0-efa2-11e8-9c47-b7137603aa53
e634be48-eec8-11e8-bbb3-bf16b851a67e	0ad9db90-efa3-11e8-8da4-379a50fc52bf
49c54b40-efa4-11e8-a66a-4f252f6e5a86	4d4fccc2-efa4-11e8-9378-e7ec426b9c2a
dae01074-eec3-11e8-9afc-4be17860c00c	7dc93c58-efa4-11e8-9378-1339aff733d3
3c35074c-ee08-11e8-9465-338bc5df4123	171271ae-efa5-11e8-b659-33d68a5debe0
33b18df0-eec8-11e8-b9e7-270ae3464cf5	6c4b58b6-efa5-11e8-9378-33255dd98476
45644e98-ecd9-11e8-8e0e-6fa75df8779e	e35670b0-eed5-11e8-b0bb-67de2d98e2ed
3c35074c-ee08-11e8-9465-338bc5df4123	6249da0a-ef4f-11e8-ad6b-076386c675bd
45644e98-ecd9-11e8-8e0e-6fa75df8779e	f4d5f5ec-eed3-11e8-b0bb-a74e36ad2c35
45644e98-ecd9-11e8-8e0e-6fa75df8779e	3e924c36-eed3-11e8-b0bb-3fdce572b7f2
33b18df0-eec8-11e8-b9e7-270ae3464cf5	34d2819e-eed2-11e8-b0bb-3fbcbc1d6649
a117a270-efa5-11e8-9001-07b2822d7463	34d2819e-eed2-11e8-b0bb-3fbcbc1d6649
fa96a184-efa5-11e8-9b1d-e3851e22611f	fde7eb18-efa5-11e8-9b1d-8f276070f0a0
45dca814-e6f0-11e8-8bc1-b363da4aeace	dde9cbd6-ee08-11e8-b1d2-572b8f794183
fa96a184-efa5-11e8-9b1d-e3851e22611f	a69a6cd8-ef9a-11e8-9ace-2fef6edbf55a
49c54b40-efa4-11e8-a66a-4f252f6e5a86	17ab0814-efa6-11e8-ac8b-3fab38ec8d3c
b7c14ab8-ee73-11e8-ba50-233c6ad09102	7cf3100e-efa6-11e8-9b1d-9bb6f883fb44
b7c14ab8-ee73-11e8-ba50-233c6ad09102	8c9a8a0a-efa6-11e8-9b1d-17aafcd08e6a
b7c14ab8-ee73-11e8-ba50-233c6ad09102	d9c5b80e-efa6-11e8-9b1d-931dc8ef2807
bbd64e16-efa7-11e8-9b1d-37f9635711fa	be578830-efa7-11e8-ac8b-e347749469c3
bbd64e16-efa7-11e8-9b1d-37f9635711fa	cb95a7d4-efa7-11e8-ac8b-df713246a9c8
f29b769c-efa7-11e8-ac8b-9b17246a5644	f60c35aa-efa7-11e8-9b1d-338abd4c1c2d
45dca814-e6f0-11e8-8bc1-b363da4aeace	4d141478-ee07-11e8-ae67-53c85dce4ba5
28f851c8-efa9-11e8-b1dc-27bf66b1795c	2b77931e-efa9-11e8-b1dc-e75c1a9acee8
b7c14ab8-ee73-11e8-ba50-233c6ad09102	86408576-efa9-11e8-855e-67d0439b6266
bacbdf66-efa9-11e8-855e-2b4716af8be5	bdb349da-efa9-11e8-b1dc-e38cfdaffb90
bacbdf66-efa9-11e8-855e-2b4716af8be5	c98a87c4-efa8-11e8-97a3-2b0cb82d3067
492019e8-ee07-11e8-8642-239c847b42a7	e96d6c62-f90f-11e8-89bd-87c28b1ae95f
c922b0ce-ee07-11e8-aa9d-131c55a142f2	6b1bc86a-efa8-11e8-97a3-5b606dfd4b38
bacbdf66-efa9-11e8-855e-2b4716af8be5	6b1bc86a-efa8-11e8-97a3-5b606dfd4b38
bacbdf66-efa9-11e8-855e-2b4716af8be5	fb2fcaee-ef9b-11e8-9826-83f920580dd0
28f851c8-efa9-11e8-b1dc-27bf66b1795c	e35b646a-efa9-11e8-8ad2-4fbfdcfb0b4f
e9819378-efa9-11e8-8ad2-bf278fa3d8f5	e35b646a-efa9-11e8-8ad2-4fbfdcfb0b4f
742cc830-efaa-11e8-8ad2-bbe233f83fdf	76051284-efaa-11e8-8ad2-8f5205ff6926
85c444a6-efaa-11e8-8ad2-c77d30692147	87d6f932-efaa-11e8-adbb-2b169db9ff63
45644e98-ecd9-11e8-8e0e-6fa75df8779e	daa01874-efaa-11e8-a938-83a977ebf756
45644e98-ecd9-11e8-8e0e-6fa75df8779e	f4f80ff6-efaa-11e8-a938-43e9badba284
45dcaad0-e6f0-11e8-8bc1-677f3b3c362f	2dba435e-efab-11e8-a060-ab419b039285
3c35074c-ee08-11e8-9465-338bc5df4123	ddf5a0b0-ef51-11e8-ad6b-b3a1f76ec510
2245933a-eec4-11e8-b546-a70043135c65	fec9434a-eade-11e8-8231-3be3240b1542
0bb8bf84-f004-11e8-8f24-e74b1ee65f0f	0e323b82-f004-11e8-9b43-f3e52e9d23d7
0bb8bf84-f004-11e8-8f24-e74b1ee65f0f	392c098a-f004-11e8-9b43-8b79b968c096
7df1e544-efa5-11e8-9378-e30c0f29d98e	ce88ec7e-f008-11e8-9b43-37753e4b59a4
54ff610c-f009-11e8-a42a-7b0db2139f6c	ce88ec7e-f008-11e8-9b43-37753e4b59a4
7df1e544-efa5-11e8-9378-e30c0f29d98e	80f70238-efa5-11e8-9378-0b8e4af30b3e
54ff610c-f009-11e8-a42a-7b0db2139f6c	80f70238-efa5-11e8-9378-0b8e4af30b3e
7df1e544-efa5-11e8-9378-e30c0f29d98e	d27a7950-ef69-11e8-afc5-439aa12d67c0
54ff610c-f009-11e8-a42a-7b0db2139f6c	d27a7950-ef69-11e8-afc5-439aa12d67c0
7df1e544-efa5-11e8-9378-e30c0f29d98e	42591762-ef66-11e8-a507-af6b067ae962
54ff610c-f009-11e8-a42a-7b0db2139f6c	42591762-ef66-11e8-a507-af6b067ae962
df63295e-ee02-11e8-9e36-17d56b662bc8	4d205232-f00b-11e8-9b43-f7734d272121
bae512ee-f00b-11e8-a42a-c76c80c8c854	543d3840-f00c-11e8-9b43-331950967e81
bae512ee-f00b-11e8-a42a-c76c80c8c854	bd32e71a-f00b-11e8-9b43-73cf6f5c7a92
7df1e544-efa5-11e8-9378-e30c0f29d98e	bd32e71a-f00b-11e8-9b43-73cf6f5c7a92
df63295e-ee02-11e8-9e36-17d56b662bc8	cdcedab0-f381-11e8-a877-13f11640591c
\.


--
-- Data for Name: links; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.links (organization_id, id, url, title, sha1, created_at, updated_at, repository_id) FROM stdin;
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	24f8a6f8-eec4-11e8-9c83-cf64a2f02829	http://bradfrost.com/blog/post/ditching-the-macbook-pro-for-a-macbook-air/	Ditching the MacBook Pro for a MacBook Air	f476ddad9a5901f42d9119a975c05a510f0e4cf4	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	ddf5a0b0-ef51-11e8-ad6b-b3a1f76ec510	https://news.ycombinator.com/item?id=18513249	US asks allies to drop Huawei | Hacker News	78f94bad9033110d31d42c226ddda7770d4c5a51	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	7da9f70e-ef58-11e8-97a8-f3303474aa70	https://blog.cloudflare.com/every-7-8us-your-computers-memory-has-a-hiccup/	Every 7.8μs your computer’s memory has a hiccup	bbd9dd348f96f8cdb612d23f7370457dfafd0323	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	d35a7362-ef5e-11e8-97a8-8f6e20dbc7c6	https://news.ycombinator.com/item?id=18517081	Every 7.8μs your computer’s memory has a hiccup | Hacker News	9adfff8209e9e812d778514df0ebb0a3420aac41	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	6249da0a-ef4f-11e8-ad6b-076386c675bd	https://www.nzherald.co.nz/business/news/article.cfm?c_id=3&objectid=12165136	US asks allies to drop Huawei	e733efe872a69d54cee546ebbefc473c0da681db	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	ae331398-ef61-11e8-add3-b39b7ea7844d	https://news.ycombinator.com/item?id=18516177	The Resistance Is Real – Why Side Projects Are So Hard | Hacker News	5b4c9c922203462ed60b5119f9d483c87525fc6e	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	42591762-ef66-11e8-a507-af6b067ae962	https://news.ycombinator.com/item?id=18518407	Just Released: Fourth National Climate Assessment Volume II | Hacker News	063e871b483afb153665d8e7a2c47cb7e1457203	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	c0077460-ef66-11e8-994e-d7feffa99f29	https://mosaicscience.com/story/climate-change-deadly-epidemic-chronic-kidney-disease/	Climate change is turning dehydration into a deadly epidemic	9bc238b72fc6176071d1c393491416955a4bc751	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	c5f9835e-ef66-11e8-994e-23d967db8097	https://news.ycombinator.com/item?id=18517323	Climate change is turning dehydration into a deadly epidemic | Hacker News	5d87a5935ed73dca817983f7e2fd96df4be2d217	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	3d4abca6-ef68-11e8-aa37-2b21c89bd76a	https://usa.streetsblog.org/2018/11/23/u-s-finally-legalizes-modern-european-style-train-cars/	U.S. Finally Legalizes Modern, European-Style Train Cars	c22ef67d6a0b72a48b4746043ffd344685e261ff	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	f44708ba-ef68-11e8-a1af-17b3db2b15ad	https://news.ycombinator.com/item?id=18518721	U.S. Finally Legalizes Modern, European-Style Train Cars | Hacker News	22d28c7a1020529710ea0e95578e82ace72bf0c0	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	d27a7950-ef69-11e8-afc5-439aa12d67c0	https://www.nytimes.com/2018/11/23/climate/us-climate-report.html	U.S. Climate Report Warns of Damaged Environment and Shrinking Economy - The New York Times	0bb492581bd1a07e9471f9a18fd6c24f5894dc93	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	d0f9df9c-ef70-11e8-9894-4771acb08d56	http://threespeedlogic.com/python-tworoutines.html	Three-Speed Logic	f4954052b3cbda10d562f097fcf3091423e18abe	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	41a0f0f8-ef73-11e8-a1b2-438674b0b6a4	https://medium.com/@davealexis/this-is-why-i-consider-the-async-await-pattern-to-be-like-a-virus-e029d95fcba1	This is why I consider the async/await pattern to be like a virus.	196612ca1f525d24b2e4a7246db9a556c35b7062	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	5e76f31c-ef73-11e8-a1b2-6bf2acf69702	https://bugs.python.org/issue22239	\nIssue 22239: asyncio: nested event loop - Python tracker\n\n	d9355e548a219961cb382f6b29d89384c9976971	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	a39f0ee8-ef73-11e8-9894-2bcad5930372	http://www.icce.rug.nl/documents/cplusplus/	The C++ Annotations	477d91d55386cf98ef84904cbb0df9dc0a92a008	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	41166536-ef74-11e8-9894-8b9f0d759075	http://www.bbc.com/travel/story/20181122-the-nine-ghost-villages-of-northern-france	The nine ghost villages of northern France	b22a3c269539efdba3fc9f5a6222398a3e7c9083	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	8a15ea4a-ef74-11e8-8aa2-6b66d2d9bdc3	https://en.wikipedia.org/wiki/Big_Bertha_(howitzer)	Big Bertha (howitzer) - Wikipedia	830f1df636991f5a0f8d1baf1387e7f3bd9808ce	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	25abbdf8-ef76-11e8-89a9-4bfd0a8e3d1e	https://www.syfy.com/syfywire/amazing-time-lapse-video-of-a-rocket-launch-seen-from-space	Amazing time-lapse video of a rocket launch… seen from space!	4fbb134e86ee7206d451904a19460cc43da8031a	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	6599de8c-ef98-11e8-99f9-779097e0bf76	https://www.reddit.com/r/tiltshift/comments/9zr3bl/saigon_vietnam/	r/tiltshift - Saigon, Vietnam	b3625d840183c014a9955217a8d4fff694c198b1	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	a9f05818-ef98-11e8-99f9-df8afcb9b764	https://www.reddit.com/r/evilbuildings/comments/9zqf91/this_photo_i_took_of_a_theme_park_entrance_in/	r/evilbuildings - This photo I took of a theme park entrance in Holland	34cc530e939b2766f59b0456666404b2c14db507	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	b6d23b5a-ef98-11e8-bf0b-6f2e1f977c5e	https://www.reddit.com/r/AbandonedPorn/comments/9zoali/black_friday_oc2500x1500/	Black Friday (oc)(2500x1500) • r/AbandonedPorn	87a571df802d2da409456acf2edb07426fa0327b	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	bf09b950-ef99-11e8-94cf-3f0981918655	https://www.reddit.com/r/TheWayWeWere/comments/9zo5x7/the_schoolbus_in_maine_1930/	r/TheWayWeWere - The schoolbus in Maine, 1930	3d20a9fc5642c73547f1e78902a3aa0c69dd33b9	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	f8396834-ef98-11e8-9ace-f30ff229ccb6	https://www.reddit.com/r/programming/comments/9zrf8j/happy_fibonacci_day_here_is_how_to_generate_a/	r/programming - Happy Fibonacci day, here is how to generate a Fibonacci sequence in PostgreSQL	cffdb231df9aacfd6910cab18334b8cb125584d4	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	1f077e1a-ef99-11e8-9ace-173fec9de1b8	https://denisgobo.blogspot.com/2018/11/happy-fibonacci-day-here-is-how-to.html	Happy Fibonacci day, here is how to generate a Fibonacci sequence in PostgreSQL 	bff6080780312be5a3668bcee09c975c1b9e52a7	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	5e94fd82-ef99-11e8-9ace-27e171b810a9	https://www.reddit.com/r/brutalism/comments/9zqam9/andrew_melville_hall_university_of_st_andrews_st/	r/brutalism - Andrew Melville Hall, University of St. Andrews, St. Andrews, Fife	4971731f94ada10d08c40b00ee5e52f44b6801d9	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	e96d6c62-f90f-11e8-89bd-87c28b1ae95f	http://localhost:3001/topics	Error	589113ec186dc84c9d132e020c2952400b215783	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	a69a6cd8-ef9a-11e8-9ace-2fef6edbf55a	https://www.reddit.com/r/HeavySeas/comments/9zomj4/aboard_a_cruise_ship_in_heavy_seas/	r/HeavySeas - Aboard a cruise ship in heavy seas.	1a837e3a87cadd3497e75067442ebf0a0b7f9c78	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	ca09f7a6-ef9a-11e8-a161-dfc86e1e0da6	https://www.reddit.com/r/TheWayWeWere/comments/9zot67/mother_and_child_after_the_hiroshima_bombing_1945/	Mother and child after the Hiroshima bombing, 1945. • r/TheWayWeWere	9aeb5e55eff21a107f4b65cc52159fc2d5d1f019	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	d601d010-ef9a-11e8-a161-6bfabe60d300	https://www.reddit.com/r/AmateurRoomPorn/comments/9zq3ul/finished_library_in_my_new_house_result_cranston/	Finished library in my new house, result! (Cranston, Rhode Island) • r/AmateurRoomPorn	e9838f15123d5765e622660f52ae289b9e237056	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	0983ddf2-ef9b-11e8-8d14-5baf275349e1	https://www.reddit.com/r/HistoryPorn/comments/9zp6nc/construction_of_the_manhattan_bridge_new_york/	r/HistoryPorn - Construction of the Manhattan Bridge New York City [991x742] Year 1903	885bb2cd5563e83c988eb1d26d707a03bdcbeb85	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	45f21cfe-ef9b-11e8-8d14-2ba1f305205e	https://www.reddit.com/r/architecture/comments/9zo0oj/the_victorian_gothic_of_th_carson_mansion_eureka/	r/architecture - The Victorian gothic of th Carson mansion, Eureka, California [building]	025969d2f624df00dfd7ac7860c803e8b366b872	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	52d97048-ef9b-11e8-886c-47ab437d4d4d	https://www.reddit.com/r/Cyberpunk/comments/9zo15h/the_final_render_of_my_blade_runner_render_video/	r/Cyberpunk - The final render of my Blade Runner render, video link in comments.	1c247bd8d1eab50709698ef85d2c51222be4b559	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	b5ce0a74-ef9b-11e8-8d14-67ba1c14dcef	https://softwarebrothers.co/blog/two-weeks-with-kubernetes-in-production/	Two weeks with Kubernetes in production	7b4f0b6ef8b812c6c0fb9c9f71deb489be99f1c1	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	bd115cf0-ef9b-11e8-8d14-c3e70a67da5f	https://www.reddit.com/r/kubernetes/comments/9znut6/two_weeks_with_kubernetes_in_production/	r/kubernetes - Two weeks with Kubernetes in production	3d347c578f3cbbd2674a36f581e8331f2fc192ac	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	bcfc7caa-ef9f-11e8-9d26-7fb83e5bfa96	https://www.reddit.com/r/brutalism/comments/9zshz7/brooklyn_army_terminal_atrium_designed_by_cass/	r/brutalism - Brooklyn Army Terminal atrium, designed by Cass Gilbert (1918) [building]	c09d3708b20dedf9acb54590c683a1c8504dcc1e	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	f171992a-ef9f-11e8-9d26-7f3d46f17c61	https://www.reddit.com/r/askscience/comments/9zqmxv/are_there_any_known_examples_of_domesticated/	r/askscience - Are there any known examples of domesticated mammals becoming extinct?	136acb1c0d8aaa13f07f5c244073ce0641a4fd4b	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	fde7eb18-efa5-11e8-9b1d-8f276070f0a0	https://www.reddit.com/r/WarshipPorn/comments/9zpssh/the_british_frigate_hms_argyll_front_japanese/	The British frigate HMS Argyll (front), Japanese destroyer Inazuma (center) and Japanese helicopter carrier Kaga take part in a joint naval drill in the Indian Ocean [2000 × 1375] • r/WarshipPorn	eba6de8d13d6575a97cf1e8cf4159bd709b27834	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	95b6e976-ee75-11e8-9c27-03de8080687c	https://www.zeit.de/digital/games/2018-11/computer-games-gdr-stasi-surveillance-gamer-crowd/komplettansicht	Video Games In East Germany: The Stasi Played Along	6f35a5350f8a62ea8896095a4d37ee1cbf2d7153	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	fec9434a-eade-11e8-8231-3be3240b1542	https://github.com/	Github	d7b3438d97f335e612a566a731eea5acb8fe83c8	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	36c2b64a-eec8-11e8-9d24-9713d9c75592	https://www.reddit.com/r/SweatyPalms/comments/9zew5v/an_encounter_with_wolves/	r/SweatyPalms - An encounter with wolves	317f9a40351f77009a52f18f87731ec3c5b6ccb3	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	98db95a6-eed0-11e8-bfb2-f7044d070ccf	https://www.reddit.com/r/RoomPorn/comments/9zcyk8/transparent_living_spaces_rising_over_the/	r/RoomPorn - Transparent living spaces rising over the treetops in a partially submerged five story glass residence accessed with a pedestrian bridge, Dallas, Texas [1925×2880]	eb302a46efcab413f681732bd73004c034294675	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	bc1160f6-ef97-11e8-aac0-272abf43dea2	https://www.reddit.com/r/UrbanHell/comments/9zmaj0/people_offering_prayers_at_river_yamuna_india/	People offering prayers at River Yamuna, India, which is frothing from industrial waste • r/UrbanHell	b301078feb9b1cd5583c9e5f4ad8e7a2caa809a5	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
b336b93e-fc14-11e8-bd87-4304c8bbc51f	77b68630-ff68-11e8-befa-c7cdcb0356a1	https://news.ycombinator.com/	Hacker Newz	f1f3bd09de659fc955d2db1e439e3200802c4645	2018-12-14 00:21:18.49165-06	2018-12-14 00:35:39.711348-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	d90e2cb2-ef9b-11e8-b219-0702492ff807	https://www.reddit.com/r/travel/comments/9zs1y5/la_alhambra_in_spain/	r/travel - La Alhambra in Spain	0f319c9643b92d1ad1cab32076f9a2e4ae80ec5e	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	fb2fcaee-ef9b-11e8-9826-83f920580dd0	https://www.reddit.com/r/golang/comments/9zqn0d/a_vpnserver_using_websockets/	r/golang - A VPN-server, using websockets	98c5ad74c743f533f8ea5be5a2a9d59ed6d43f55	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	d17b9844-ef9c-11e8-9826-e79b3460119d	https://www.reddit.com/r/MachinePorn/comments/9zsy0a/big_brutus_second_largest_electric_shovel_in/	r/MachinePorn - Big Brutus - Second largest electric shovel in Kansas. [OC] [1344x750]	4a6bef7f3aaa8f2c48846dac96d95daee13413ab	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	dbdbe682-ee0b-11e8-b1d2-677e1f1f2f91	https://www.nytimes.com/2018/11/21/us/paradise-fires-emergency-alerts.html	A Frantic Call, a Neighbor’s Knock, but Few Official Alerts as Wildfire Closed In - The New York Times	cc3a42083f0094cc10f53aa49323f26472cc12a7	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	f162e5a4-ef9c-11e8-adea-33d098c6ae27	https://www.reddit.com/r/WildernessBackpacking/comments/9zqbyi/near_the_summit_of_mt_namuli_gurue_zambezia/	r/WildernessBackpacking - Near the summit of Mt Namuli, Gurue, Zambezia Mozambique - 5 day trip - October 2017	2b2c9cdf814f833f0cd26e9e29a356f93ed5188b	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	021f5a8a-ef9d-11e8-adea-87ab9e69bacf	https://www.reddit.com/r/oddlysatisfying/comments/9zptmj/this_suspension/	r/oddlysatisfying - This suspension	7a7267c5a41e0ef1bdbd795e2b2a2bb40ab840cb	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	1a960a32-ef9d-11e8-8487-ebf308f47aaf	https://www.reddit.com/r/AmateurRoomPorn/comments/9zrt7e/autumn_vibes_in_the_kitchen_wales_uk/	r/AmateurRoomPorn - Autumn vibes in the kitchen (Wales, UK)	d17c507b53261434c0b83a654f5fa0a3cf929034	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	576bd562-ef9e-11e8-843e-7b1b11140f00	https://www.reddit.com/r/aviation/comments/9zrb2k/the_giants_first_landing_in_australia/	r/aviation - The Giant's First landing in Australia	fe5163eb43823a15571b9aa89c74a76f333baddd	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	67e9f2e8-ef9e-11e8-ac40-7341c4fc3aaf	https://www.reddit.com/r/blackmagicfuckery/comments/9zqrrq/engine_powered_by_the_heat_of_a_hand/	r/blackmagicfuckery - Engine powered by the heat of a hand	6ced3783dd365fbb127e3c5447cca8ef70bb5937	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	2ac60822-ee75-11e8-a81c-5ff309b5de84	https://news.ycombinator.com/item?id=18509243	Pembrokeshire treasure hunter unearths Celtic chariot | Hacker News	75e9db9b511f7591c5a8beb0b14f5df14c51ee0f	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	cbd1adc6-eec9-11e8-bbb3-db2b8881d52e	https://www.reddit.com/r/TheWayWeWere/comments/9zd5de/washington_dc_1942/	r/TheWayWeWere - Washington, D.C., 1942	d08665c595ab368152c62e268d0fb13d6abcfb8e	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	be08d798-ef9e-11e8-843e-b35f24bdee27	https://themasters.io/blog/posts/approaches-to-handling-service-objects-returns	4 approaches to handling service objects' returns - The Masters	a21a5b77d812db8fc42a7b62b888f8dbc37a0675	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	29dac70a-efa0-11e8-9d26-67a7dfdfebcc	https://www.reddit.com/r/science/comments/9zo830/dna_vaccine_reduces_both_toxic_proteins_linked_to/	r/science - DNA vaccine reduces both toxic proteins linked to Alzheimer’s: A vaccine delivered to the skin prompts an immune response that reduces buildup of harmful tau and beta-amyloid in mice modeled to have Alzheimer’s disease. Scientists say the vaccine is getting close to human trials.	6471ab695ecad6d9228d992a183adea82173158e	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	8e94ca2c-eecb-11e8-bbb3-075606f0461a	https://www.reddit.com/r/WeatherGifs/comments/9zjccl/massive_dust_storm_in_australia_has_turned_the/	r/WeatherGifs - Massive dust storm in Australia has turned the skies dark red	cdaacbdc0a60172e5f0729ca8407920b47bfdbf7	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	edb27e1e-eecb-11e8-b93f-032a1d1e8730	https://www.reddit.com/r/Cyberpunk/comments/9ziog0/this_art_from_andre_grippe/	r/Cyberpunk - This art from Andre Grippe.	1d2c50e4f00a8fbf19427ed5d5eae4be3e56b950	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	fab0e8fc-eecc-11e8-bbb3-170f613af621	https://www.reddit.com/r/AnimalsBeingBros/comments/9zhw8k/shark_begging_to_be_pet/	r/AnimalsBeingBros - Shark begging to be pet	f8d9c0210ef249f0ab0f7db41aa5f56ca3eaefd0	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	693c13d6-eece-11e8-b7a6-eb18e45495e0	https://rhonabwy.com/2018/11/22/review-of-using-helm-to-package-and-host-applications/	Review of using Helm to package and host applications	0b0f1ff8fe114e4737b4eb4ab5ce45e41a87358e	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	8a77d3f8-eed0-11e8-b0bb-afd686b6376e	https://www.reddit.com/r/spaceporn/comments/9zdikv/tracks_on_mars_5812x2302/	r/spaceporn - Tracks on Mars [5812x2302]	a94669bd28c1a30743eaf7bc1c27e6f84cf28859	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	2436f5dc-eed1-11e8-b0bb-f78739f8edef	https://www.reddit.com/r/interestingasfuck/comments/9zhlyv/a_deep_abyss_of_fish/	A deep abyss of fish • r/interestingasfuck	10a4340e320aa56bf3f89b1feb013eb403931a11	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	34d2819e-eed2-11e8-b0bb-3fbcbc1d6649	https://www.reddit.com/r/EarthPorn/comments/9zh9x8/i_sat_and_waited_for_3_hours_at_7c_for_the_clouds/	I sat and waited for 3 hours at -7°C for the clouds to clear for this. Matterhorn, Swiss Alps [OC][6000x3375] • r/EarthPorn	7fe3e9cbcacde6b4e2b2dbec618061085cf57910	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	3e924c36-eed3-11e8-b0bb-3fdce572b7f2	https://www.reddit.com/r/aww/comments/9zip8c/ever_seen_a_newborn_sloth/	r/aww - Ever seen a newborn sloth?	50ba87a558c438504623b80acad72b851f2705ef	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	f4d5f5ec-eed3-11e8-b0bb-a74e36ad2c35	https://www.reddit.com/r/likeus/comments/9zjc78/bath_time/	r/likeus - Bath time!	cba1959355a7c10af7eb32471e8f4451a2da89d9	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	e35670b0-eed5-11e8-b0bb-67de2d98e2ed	https://www.reddit.com/r/WTF/comments/9zj4em/the_sound_of_nine_opossum_joeys_chomping_bananas/	The sound of nine opossum joeys chomping bananas • r/WTF	b947841fc3ec18ece857b3af8865a51b067a0829	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	3ea8d98a-eed6-11e8-8de7-8f1ea67e5d8b	https://www.theguardian.com/uk-news/2017/mar/29/king-george-v-was-murdered-not-euthanised	King George V was murdered, not euthanised | Letters	331fc85afdf109950969fde7ad5dbc0807e6dd06	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	08c60046-ef43-11e8-b802-4bc00ef1d22a	https://blog.erratasec.com/2018/11/some-notes-about-http3.html	Some notes about HTTP/3	391382fab60fbd0b03e1f08bbad9583bf3909c35	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	aae24886-ef9c-11e8-9826-1b3bb08fdbbd	https://github.com/skx/simple-vpn/	skx/simple-vpn	8edaf247c609251dfc6af0198eddbfbf4a08c13c	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	be804548-ef72-11e8-9894-a3d8d275a791	https://news.ycombinator.com/item?id=18516121	Tworoutines in Python | Hacker News	55964a8132b229ee1175af7a1175aaa703267377	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	4724d7f6-efa0-11e8-9c47-53f873b5c385	https://www.reddit.com/r/aviation/comments/9ztliy/after_50_years_she_is_still_the_fastest_bird_in/	r/aviation - After 50+ Years, She is Still the Fastest Bird in the Sky	157b61ce9815a5ee1df444bb571b6835489ccdea	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	709b13de-efa0-11e8-9c47-fb7bab4fce38	https://www.reddit.com/r/EngineeringPorn/comments/9zrk1w/notice_the_nous_glow_of_cherenkov_radiation/	r/EngineeringPorn - Notice the nous glow of Cherenkov radiation	6c8a0f7871c2a8b21d444cfc54c7d1d8daa8952b	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	b42c6620-efa0-11e8-973a-c3ca26fd0256	https://www.nytimes.com/2018/11/23/opinion/sunday/red-dead-redemption-2-fallout-76-video-games.html	Opinion | Red Dead Redemption 2 Is True Art - The New York Times	f2a177d8d4842faa710f5cee095652650c205c76	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	a839c370-efa1-11e8-9c47-c7961281ba96	https://www.reddit.com/r/Suburbanhell/comments/9zu3dk/the_coachella_valley_when_you_want_to_live_in_the/	r/Suburbanhell - The Coachella Valley - When you want to live in the middle of the desert 120 miles outside of Los Angeles	f2d8616b4a01852121cbb1a6620b20eda9212a4a	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	897061a6-ee07-11e8-a945-a32c887b43c4	https://www.cia.gov/library/center-for-the-study-of-intelligence/csi-publications/books-and-monographs/psychology-of-intelligence-analysis/art4.html	Chapter 1  — Central Intelligence Agency	777771953559d7af66cf4e4c2d213515a2658be6	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	f2326d7e-efa1-11e8-973a-0b18966add17	https://www.reddit.com/r/aviationmaintenance/comments/9zrh9g/my_thanksgivingone_of_three_to_change/	r/aviationmaintenance - My Thanksgiving..one of three to change	e2423c50a088f6a4cdbbad062d0cc6dd8a20020c	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	534cb1f0-efa2-11e8-973a-bb57dbf1dc21	https://www.reddit.com/r/Futurology/comments/9znwqd/scientists_may_have_found_a_way_to_treat_cancer/	r/Futurology - Scientists may have found a way to treat cancer without chemotherapy by replicating our body's own self-destruct system - scientists from the US recently discovered a genetic "kill code" in cells that could theoretically be used to treat cancer without chemotherapy.	ec72e0f539c76149c67c0eeb7e8c5ffa23d59d49	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	783e5e46-efa2-11e8-9c47-fbb892f95289	https://www.reddit.com/r/WarshipPorn/comments/9zptjo/f_35_on_a_lift_on_hms_queen_elizabeth_1800_1125/	r/WarshipPorn - F 35 on a lift on HMS Queen Elizabeth [1800 × 1125]	2b8facd0b97925ad99285f7697bcd1deb44c22dd	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	c02485f0-efa2-11e8-9c47-b7137603aa53	https://www.reddit.com/r/HistoryPorn/comments/9zmom8/the_battle_of_kursk_july_1943_1242x735/	r/HistoryPorn - The Battle of Kursk, July 1943 [1242x735]	48a205093a295f743c4597aa91c640b938227400	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	0ad9db90-efa3-11e8-8da4-379a50fc52bf	https://www.reddit.com/r/urbanexploration/comments/9zoh26/inside_a_train_tunnel_oc1638x2048/	r/urbanexploration - Inside a train tunnel [OC][1638x2048]	373d2c1680122ad83da422ead6b394b37d1196dc	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	2327d332-efa3-11e8-8b9a-7b6e3edd5934	https://www.reddit.com/r/space/comments/9ztp1s/beneath_antarcticas_ice_is_a_graveyard_of_dead/	r/space - Beneath Antarctica’s Ice Is a Graveyard of Dead Continents. Data from a European satellite has revealed the tectonic underworld below the frozen southernmost continent.	92ef5f915d0747b9a6492744ab6a3f5edce3b13f	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	4d4fccc2-efa4-11e8-9378-e7ec426b9c2a	https://www.reddit.com/r/news/comments/9zop96/denmark_germany_netherlands_and_finland_join/	r/news - Denmark, Germany, Netherlands and Finland join countries halting weapons sales to Saudi Arabia	0662bd06379ecc3742b8b16922c6aa5230def15b	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	7585ca9a-ee7b-11e8-b88e-83b7e9821fe1	https://stackoverflow.com/questions/9571392/ignoring-time-zones-altogether-in-rails-and-postgresql/9576170	Ignoring time zones altogether in Rails and PostgreSQL - Stack Overflow	6bed84cd98fadd2022fc76ac7c6357770830103a	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	b0633868-ee7c-11e8-b88e-af9924a3aaef	https://dba.stackexchange.com/questions/156980/create-a-trigger-on-all-the-last-modified-columns-in-postgresql	Create a trigger on all the last_modified columns in PostgreSQL - Database Administrators Stack Exchange	e5b854428d7160a42c596794fb9e4dd7a8e3a5b9	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	5e4cd742-ee7b-11e8-bb6f-e33b2e13652c	https://en.wikipedia.org/wiki/Daylight_saving_time	Daylight saving time - Wikipedia	67b179c6f5a078f37513a869f979cf72625d65a8	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	7b077bce-eec8-11e8-9ff0-2b69edcddd6b	https://www.reddit.com/r/UrbanHell/comments/9zdnyd/grenoble_france/	Grenoble, France • r/UrbanHell	0e00e430111996d2ea6bdfdb8e20bdb1ffd1b71c	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	9526e0f8-eec8-11e8-849a-c34e40cc2227	https://www.reddit.com/r/HistoryPorn/comments/9zfcbj/the_last_hussar_august_von_mackensengerman_field/	r/HistoryPorn - "The Last Hussar" August von Mackensen,German field marshal in World War I, in Leib-Husaren-Regiment uniform, circa 1914 [colorized] [743x1000]	38060a559ed919da52ac577a21864e4919c406a9	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	5b0b260e-ee73-11e8-ba50-5ffa77fcbe57	https://gcemetery.co/	The Google Cemetery	58f88a6810dda73f0504935c5ca71349aa903bd2	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	7dc93c58-efa4-11e8-9378-1339aff733d3	https://www.reddit.com/r/deepdream/comments/9zrmvl/red_leaf_viper_4k_blending_3a3x3_4b3x3_reduce/	Red Leaf Viper (4k) blending 3a-3x3 & 4b-3x3_reduce (mostly) • r/deepdream	d3d1541bb90c7cb8e8f67f659db42989167ad440	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	171271ae-efa5-11e8-b659-33d68a5debe0	https://www.reddit.com/r/ProgrammingLanguages/comments/9zrrj6/are_there_any_programming_languages_that_have/	Are there any programming languages that have "configurable" typing systems? • r/ProgrammingLanguages	b1fdeba4f2403771c6f8e31c133de2309223dbed	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	6c4b58b6-efa5-11e8-9378-33255dd98476	https://www.reddit.com/r/MostBeautiful/comments/9zrto7/we_travelled_halfway_across_the_world_to_visit/	r/MostBeautiful - We travelled halfway across the world to visit Yosemite and it was all worth it.	bc3aae9dcb56d935cbde68cceda591e299639648	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	80f70238-efa5-11e8-9378-0b8e4af30b3e	https://www.reddit.com/r/science/comments/9zrfut/the_4th_national_climate_assessment_in_the/	r/science - The 4th National Climate Assessment: "In the absence of more significant global mitigation efforts, climate change is projected to impose substantial damages on the U.S. economy, human health, and the environment."	3f4bc13b2c5daa0159875b95f34c1840a4a40e1b	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	dcaa107c-ef99-11e8-9ace-cb69e62e1dae	https://www.reddit.com/r/CozyPlaces/comments/9zouxv/cozy_for_the_winter/	r/CozyPlaces - Cozy for the Winter	633fc629dbe7be82b43e227e9299fef31d81d3c5	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	a18d2000-eec3-11e8-9afc-138d16aeec63	https://www.chemistryworld.com/news/world-first-as-wind-turbine-upgraded-with-high-temperature-superconductor/3009780.article	World first as wind turbine upgraded with high temperature superconductor	096f75a6bc627d87a4c89c8d46726ba2c6837613	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	7cf3100e-efa6-11e8-9b1d-9bb6f883fb44	https://www.reddit.com/r/TheWayWeWere/comments/9zrwfp/april_1941_mr_and_mrs_lemuel_smith_and_their/	r/TheWayWeWere - April 1941. "Mr. and Mrs. Lemuel Smith and their younger children in their farm house, Carroll County, Georgia." Acetate negative by Jack Delano	5e862b8b41a6fa71a2d8510c30ddca4cabb5dd67	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	8c9a8a0a-efa6-11e8-9b1d-17aafcd08e6a	https://www.reddit.com/r/TheWayWeWere/comments/9zua16/swedish_tourist_bus_at_a_gas_station_in_austria/	r/TheWayWeWere - Swedish tourist bus at a gas station in Austria (1953)	371237553b0431eac11435ffc4e19578a7c8218b	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	86408576-efa9-11e8-855e-67d0439b6266	https://www.reddit.com/r/todayilearned/comments/9zp6ae/til_that_the_first_woman_to_run_for_president_of/	r/todayilearned - TIL that the first woman to run for President of the United States was Victoria Woodhull in 1872, 50 years before women could vote. She had Frederick Douglass as her running mate, and spent election day in jail due to being arrested for obscenity.	27cd45fc8df72cec25a13af6e775fa1b2afe6aae	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	bdb349da-efa9-11e8-b1dc-e38cfdaffb90	https://www.reddit.com/r/golang/comments/9zsxrd/spakinawk_easy_awkstyle_text_processing_in_go/	r/golang - spakin/awk: Easy AWK-style text processing in Go	32289938ca287d851f782d117369147c6eaa9568	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	3e6c6fbe-ee08-11e8-bb66-77b3e2753eca	https://www.inkandswitch.com/slow-software.html	Slow Software	dc9339c15d9b66d243ac81f37cc527ed207f20f8	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	9dad3f1c-ee08-11e8-bb66-735b417eb99b	https://www.zdnet.com/article/popular-dark-web-hosting-provider-got-hacked-6500-sites-down/	Popular Dark Web hosting provider got hacked, 6,500 sites down | ZDNet	711a56a337981ae0de4929d8de1b9a9ff1534034	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	c4cda884-ee08-11e8-b1d2-db8198e90692	https://news.ycombinator.com/item?id=18504490	Popular Dark Web hosting provider hacked, 6,500 sites down | Hacker News	f24fd9588600f1ca5974061fcab97a38593c9f82	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	dde9cbd6-ee08-11e8-b1d2-572b8f794183	https://phys.org/news/2018-11-dog-cow-horse-pig-rabbit.html	The taming of the dog, cow, horse, pig and rabbit	040c320ad8cc0bdba35172df0c1316e123f880b2	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	d9c5b80e-efa6-11e8-9b1d-931dc8ef2807	https://www.reddit.com/r/TheWayWeWere/comments/9zud0r/santa_monica_ca_1966/	r/TheWayWeWere - Santa Monica, CA - 1966	dc3db5ff3bf3a7553fa24ba703c974fe5bb17c0b	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	e35b646a-efa9-11e8-8ad2-4fbfdcfb0b4f	https://www.reddit.com/r/travel/comments/9zmq6v/view_of_largest_lake_of_nepal_from_murma_top_took/	r/travel - View of Largest Lake of Nepal from Murma Top. Took 2 hours of climbing from Rara Lake.	df7d17d17fb2826cc94402f53ea7fa1b61de3c31	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	76051284-efaa-11e8-8ad2-8f5205ff6926	https://meurer.xyz/post/2018-11-18-euclidean-spaces/	null pointer	152edbfa1b8512d3da62ddfde297cc1a39724c29	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	87d6f932-efaa-11e8-adbb-2b169db9ff63	https://news.ycombinator.com/item?id=18519605	Going to university does not broaden the mind | Hacker News	463de150084548a603c1b12679243e2f8da681cd	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	daa01874-efaa-11e8-a938-83a977ebf756	https://phys.org/news/2018-11-scientists-huge-ancient-herbivore.html	Scientists find remains of huge ancient herbivore	e1294f70673e2fb0c268bcf12eeffdb8472fac58	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	8ccbece0-f022-11e8-ba54-a701cebc41cb	https://medium.com/@vCabbage/go-installing-multiple-go-versions-from-source-db5573067c	Go: Installing Multiple Go Versions from Source – Kale Blankenship – Medium	42ea5cad85db9f44d62b2c54de099b34181d7883	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	9bdd8356-f022-11e8-8d18-a351a2cbaf74	https://cloud.google.com/appengine/docs/flexible/go/quickstart	Quickstart for Go in the App Engine Flexible Environment  |  App Engine flexible environment for Go\n       |  Google Cloud	c820a8220cb9e4c26c385b7dea9e70a0bf104871	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	ce88ec7e-f008-11e8-9b43-37753e4b59a4	https://www.nytimes.com/2018/11/23/climate/highlights-climate-assessment.html	What’s New in the Latest U.S. Climate Assessment - The New York Times	49b4dfd0f0df2fe4bd4039e260be40446a35000c	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	bd32e71a-f00b-11e8-9b43-73cf6f5c7a92	https://www.vulture.com/2018/11/12-monkeys-why-terry-gilliams-movie-is-so-relevant-today.html	12 Monkeys Is the Apocalypse Movie We Need Right Now	477cdf49f77821d6b72c145a6159170d3afe1b79	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	543d3840-f00c-11e8-9b43-331950967e81	https://news.ycombinator.com/item?id=18519444	‘12 Monkeys’: Why Terry Gilliam’s Movie Is So Relevant Today | Hacker News	99cc5c3d88aa220f26fdbe711bc60cf246ef6818	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	92ee44da-ee0e-11e8-be24-db545f10e1f9	https://www.reddit.com/r/AskReddit/comments/9z40sm/former_undercover_cops_of_reddit_what_is_the/	r/AskReddit - Former undercover cops of Reddit, what is the craziest thing you had to do to not blow your cover?	29e521205b6f1b421c35f49c95c774f6358e2578	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	bc615504-ee73-11e8-ba50-2ffe13c3897c	https://www.bbc.com/news/uk-wales-46294000	Treasure hunter finds buried chariot	d611bf8ea1acc580f7c86e11ef5a41ad12015ad6	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	29fb0312-eec4-11e8-b546-ff4cb3cdb805	https://news.ycombinator.com/item?id=18511184	Ditching the MacBook Pro for a MacBook Air | Hacker News	04308c9ff1d9c2c8ec914dbd91320b90dc05e2b5	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	6977c746-eec4-11e8-b546-1b758c7e4e34	https://petapixel.com/2018/11/20/the-story-behind-that-ikea-photo-of-amsterdam/	The Story Behind That IKEA Photo of Amsterdam	b185c53b2e56e4ab195396927602605caaa0f6e6	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	0a77ebea-ee01-11e8-86f0-5b6a2394f2e2	https://golang.org/pkg/log/	log - The Go Programming Language	51afa0ea2bd70aa9f40b2d0a65a2178786252a4e	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	f871fe4c-ee76-11e8-947a-77b30d881a83	https://amp.rs/	Amp - A complete text editor for your terminal	aa6081f1f941a408423da0e5ad660bc46120caf3	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	10120416-eadf-11e8-8231-db1081f8c4fc	https://www.google.com/	Google	595c3cce2409a55c13076f1bac5edee529fc2e58	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	0e7ae514-ee77-11e8-947a-77f1b1a0b3c8	https://news.ycombinator.com/item?id=18502196	Amp – A complete text editor for the terminal | Hacker News	9b21432aeaefa004e9e5853aee231cb20e7b5960	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	1e60d664-ee77-11e8-b119-a7d48fac799c	https://www.1843magazine.com/culture/look-closer/inside-the-court-of-ashurbanipal-king-of-the-world	Inside the court of Ashurbanipal, king of the world	3b30634daed117864302a360d060801d5ad69c1b	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	25db63d0-ee79-11e8-b119-bf30e8234ce4	https://news.ycombinator.com/item?id=18508530	Inside the court of Ashurbanipal, king of the world | Hacker News	46131418147a3bb996d582d5b240543c938532b7	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	0d7fbb5a-ee07-11e8-8995-3b56ae45c0b3	https://news.ycombinator.com/item?id=18504300	My hiring experience as a submarine sonar operator in the Norwegian Navy | Hacker News	55182389cc4f963bc6e8e1821689d8e5d5b82a78	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	1dbe632c-ee07-11e8-8995-77edd669ec46	https://www.brautaset.org/articles/2018/submarine-sonar-hiring.html	My hiring experience as a submarine sonar operator in the Norwegian Navy	6b12407e657f1bf5cef7782c4300232a82a58b8f	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	4d141478-ee07-11e8-ae67-53c85dce4ba5	https://www.scientificamerican.com/article/silent-and-simple-ion-engine-powers-a-plane-with-no-moving-parts/	Silent and Simple Ion Engine Powers a Plane with No Moving Parts	e38e13d1153527faccae187642938fe47bf0a4c3	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	50cfc824-ee79-11e8-98f9-e3204b163fba	https://news.ycombinator.com/item?id=18497985	Gabor Maté on Addiction to Ideology and Social Media [video] | Hacker News	fc69c61c8890fd13d64fc73936a3a443b47bf80c	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	d495b904-eec7-11e8-9b89-cfa68a7d207c	https://www.reddit.com/r/gifs/comments/9zgyim/heres_what_a_rocket_launch_looks_like_from_the/	r/gifs - Here's what a rocket launch looks like from the International Space Station	e89e5a51e67d5498a631237689ea8c6bf24f9c02	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	08791956-ef67-11e8-994e-ff881bf0d97e	https://news.ycombinator.com/item?id=18514322	The new Yahoo? Facebook should heed the lessons of internet history | Hacker News	be02f5e290dd40100142532a9ef95f5a1160decb	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	caf89828-ee07-11e8-aa9d-a7c515f0a30c	https://techcrunch.com/2018/11/21/amazon-admits-it-exposed-customer-email-addresses-doubles-down-on-secrecy/	Amazon admits it exposed customer email addresses, but refuses to give details	dadd4fd52333e89fa9c6c5db9f435287b3ce9652	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	959d07be-ee79-11e8-ad4e-c32f3bd6fb1c	https://blog.georgovassilis.com/2016/04/16/advanced-web-security-topics/	Advanced web security topics	afc550885532a243736cb091da38a6f61355b3da	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	cc289d98-ee79-11e8-ad4e-4f1b31a97790	https://news.ycombinator.com/item?id=18508211	Advanced web security topics | Hacker News	50261f60471ffaa98f31a098faa4bd5c548141dc	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	ea862bca-ee79-11e8-a9cb-afc3fd6990fe	https://news.ycombinator.com/item?id=18507407	Ask HN: Why not more hiring of junior devs, then on-the-job-training? | Hacker News	46d0d0d20883d2b5b17ef02daa0deecfea379e0d	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	b3d9e700-ee7a-11e8-bb6f-17c10d49e67b	https://dba.stackexchange.com/questions/107475/how-to-best-store-a-timestamp-in-postgresql	datatypes - How to best store a timestamp in PostgreSQL? - Database Administrators Stack Exchange	c76e1c494f1e64802b06c4be93279d3d14fe9ae8	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	35334102-ee7b-11e8-bb6f-2fed4f695d00	https://stackoverflow.com/questions/28872761/time-zone-storage-in-data-type-timestamp-with-time-zone/28876266#28876266	sql - Time zone storage in data type "timestamp with time zone" - Stack Overflow	d5aebe71c77c9f1b658c1f14c19c24d87e0987ea	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	c5422792-eec4-11e8-b546-238f1404f38b	https://news.ycombinator.com/item?id=18512268	The Story Behind the IKEA Photo of Amsterdam | Hacker News	471a194d3441e7e0e5e033ec5340824efd3f4a02	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	ebfe3a74-eec4-11e8-8bc1-7f229ad8775a	https://gitless.com/	Gitless	cc1d25957da9ccfc58dfdb77df2c3dce67ad448a	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	0fc5c936-eec5-11e8-8bc1-db5830baf62b	https://github.com/developit/htm	developit/htm	d9411e0f1660fbf9cb5288f8321839e36881ff9d	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	1f3ba264-eec5-11e8-8bc1-0b72db06d3fe	https://news.ycombinator.com/item?id=18510922	Hyperscript Tagged Markup: JSX alternative using standard tagged templates | Hacker News	fe4fc9777cd180d83340c76ac0198ac2f8e8ac43	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	3288d788-eec5-11e8-8bc1-c3b3c5902d12	https://news.ycombinator.com/item?id=18512294	Goboy: Multi-Platform Nintendo Game Boy Color Emulator Written in Go | Hacker News	8fb72801e6485c9498d5037de9f22bcc08d2534b	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	4043f88a-eec5-11e8-baaa-e3dceb8d4fbf	https://news.ycombinator.com/item?id=18512304	Thank you to dang and sctb | Hacker News	07405b685c4b0d2812ee39a5cd0e5f8b6f02cb7a	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	65ea947c-eec5-11e8-baaa-dfd390996a29	https://www.nytimes.com/2018/11/11/us/california-fire-paradise.html	Forced Out by Deadly Fires, Then Trapped in Traffic - The New York Times	cbd65f69dc5ab576f8d108992c265fe86ea06f54	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	433cb986-eec6-11e8-baaa-47d5d38f7548	https://news.ycombinator.com/item?id=18511334	Forced Out by Deadly Fires, Then Trapped in Traffic | Hacker News	d5cb0af8101cb55fcc3f0ad72df61741ff874262	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	5ce449a0-eec9-11e8-bbb3-9f3df9230ba8	https://www.reddit.com/r/AbandonedPorn/comments/9zdlcd/an_abandoned_place_in_wales_1067_x_1600/	r/AbandonedPorn - An abandoned place in Wales [1067 x 1600]	fbd8f9b355309b5c5b0f7cf331a6642ca7747a4d	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	8b224972-ef9e-11e8-843e-13ebe9173a32	https://www.reddit.com/r/ruby/comments/9znqyg/4_approaches_to_handling_service_objects_returns/	r/ruby - 4 approaches to handling service objects' returns	fd18973c4b2b47639219551008bad33caf7ab60e	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	17ab0814-efa6-11e8-ac8b-3fab38ec8d3c	https://www.reddit.com/r/Futurology/comments/9znlg7/china_blacklists_millions_of_people_from_booking/	r/Futurology - China blacklists millions of people from booking flights as dystopian 'social credit' system introduced	b6618fb447dd89100c6884cfde2ad79a5a4991b4	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	56efb816-ef43-11e8-b802-6718cf34ed85	https://news.ycombinator.com/item?id=18517047	Some notes about HTTP/3 | Hacker News	baf6c96dbbe0f0f2d601efbcd72ec64f788796c3	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	92c8dcd6-ef4e-11e8-a49e-2fff1f9fdf1e	https://www.propublica.org/article/you-snooze-you-lose-insurers-make-the-old-adage-literally-true	You Snooze, You Lose: Insurers Make The Old Adage Literally True — ProPublica	eb3515c051b8a3afe0b5b32f6e3e0885e1f2671d	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	4d205232-f00b-11e8-9b43-f7734d272121	https://news.ycombinator.com/item?id=18520403	How to Talk to People, According to Terry Gross | Hacker News	efbaedc200bb244b4acfc50d9f02038c3c2b5071	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	b6f1e908-eec8-11e8-93f6-7fcf36c56f5f	https://www.reddit.com/r/brutalism/comments/9zdmjm/this_building_is_awesome/	r/brutalism - This building is awesome	ae13a13af2b67a97a8a0cab8a5f0eae41cdf6be9	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	dce3a12e-eec3-11e8-bfbe-0f6be7d8bb14	https://www.nytimes.com/interactive/2018/11/21/nyregion/new-york-storefronts-mystery-font.html	The Mystery Font That Took Over New York - The New York Times	2bfaaae9baf924fd6e9f3bbf9158978183d0ae08	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	8f7bc29c-ef48-11e8-af6a-4f8876bdaa9c	https://newworldeconomics.com/what-a-real-train-system-looks-like/	What A Real Train System Looks Like | New World Economics	6449179f479046ca9c99c88ac90f4fbe6d0d0809	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	32f0914c-ef61-11e8-add3-bfa4399ab606	http://davemart.in/resistance/	The Resistance is Real	2f6b7b17f8b24b4c3c12fe1ee3bca1a4734ae681	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	3a680f7c-efa2-11e8-9c47-c74f5bf75acf	https://www.reddit.com/r/gifs/comments/9zq6p7/this_dog_jumps_better_than_my_lifes_going/	This dog jumps better than my life's going... • r/gifs	bdc99c7197fea6c15f222358eba9d42bf54a9282	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	cdcedab0-f381-11e8-a877-13f11640591c	https://github.com/emwalker/digraph/projects/1	emwalker/digraph	c9cca13b7199e43877f80f12f023ec987d8340f9	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	e9b26516-eec8-11e8-bbb3-8bd13fdf2fff	https://www.reddit.com/r/tiltshift/comments/9zgxae/ecity/	r/tiltshift - E-City	eb83a85354a901f32c74741584822cfa6fe16125	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	dfb3b118-ef2d-11e8-a97b-c3b43a9a195b	http://graphql-ruby.org/subscriptions/pusher_implementation	GraphQL - Pusher Implementation	aa8f6ef25a2863ae1c4a16e409b6f73116870f02	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	be578830-efa7-11e8-ac8b-e347749469c3	https://www.engadget.com/2018/11/23/nasa-spacex-crew-dragon-jan-7-test-flight-iss/	NASA sets a date for first SpaceX crew capsule test flight	a7b1d4c51ff193da033c8af9f5c18a7ee2c55a99	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	cb95a7d4-efa7-11e8-ac8b-df713246a9c8	https://www.reddit.com/r/space/comments/9ztbec/nasa_sets_a_date_for_first_spacex_crew_capsule/	r/space - NASA sets a date for first SpaceX crew capsule test flight	158ebbb3a3a78a778340801040bdbb783243b928	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	f60c35aa-efa7-11e8-9b1d-338abd4c1c2d	https://www.reddit.com/r/pics/comments/9znuao/a_park_in_kazakhstan/	A park in Kazakhstan • r/pics	e8c1da97fcb4bb75857ae9ee22f11f7add5b87f3	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	6b1bc86a-efa8-11e8-97a3-5b606dfd4b38	https://github.com/fharding1/pwnedpass	fharding1/pwnedpass	bc300c1f1c95dd12d863cf97fe5091577dda23bc	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	c98a87c4-efa8-11e8-97a3-2b0cb82d3067	https://www.reddit.com/r/golang/comments/9zri71/good_resources_for_testing_in_go/	Good resources for testing in Go • r/golang	374e783a56a5c7abd4616cd4b65622f860d26025	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	2b77931e-efa9-11e8-b1dc-e75c1a9acee8	https://i.redd.it/ch80zh0445021.png		fe39fc8cad84eecc603cb8d83eaa652784f0342b	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	f4f80ff6-efaa-11e8-a938-43e9badba284	https://news.ycombinator.com/item?id=18515982	Scientists find remains of huge ancient herbivore | Hacker News	e6933eaa64f22e280b76534fee72dd2fb2566ae5	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	2dba435e-efab-11e8-a060-ab419b039285	https://www.nature.com/articles/s41586-018-0708-8	Hemimastigophora is a novel supra-kingdom-level lineage of eukaryotes	8ad77f8dc82ceacfed1d322c9d8c94c322bd3f87	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	0e323b82-f004-11e8-9b43-f3e52e9d23d7	https://news.ycombinator.com/item?id=18521546	There is more to high house prices than constrained supply | Hacker News	8a08907652f2d68d58157c4fb9211e3af864c84f	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	392c098a-f004-11e8-9b43-8b79b968c096	https://www.economist.com/finance-and-economics/2018/11/24/there-is-more-to-high-house-prices-than-constrained-supply	There is more to high house prices than constrained supply - Free exchange	1bb253d5007bce584b312830fb44c18932be662e	2018-12-08 22:52:44.727635-06	2018-12-14 00:28:36.528491-06	b337077c-fc14-11e8-bd87-4bb3a707bd88
\.


--
-- Data for Name: organization_members; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.organization_members (organization_id, user_id, owner) FROM stdin;
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	4e92536e-fb72-11e8-b625-13a08229cacc	t
b336b93e-fc14-11e8-bd87-4304c8bbc51f	4e92536e-fb72-11e8-b625-13a08229cacc	t
\.


--
-- Data for Name: organizations; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.organizations (id, name, created_at, updated_at, login, description, public, system) FROM stdin;
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	General	2018-12-08 22:52:44.727635-06	2018-12-09 18:44:07.08699-06	wiki	The default organization when an organization is not provided	t	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	system:default	2018-12-09 18:44:07.08699-06	2018-12-09 18:44:07.08699-06	emwalker		f	t
\.


--
-- Data for Name: repositories; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.repositories (id, organization_id, name, owner_id, system) FROM stdin;
74d371f6-fc0e-11e8-b94e-2b8c1a2e2e6c	45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	General collection	4e92536e-fb72-11e8-b625-13a08229cacc	t
b337077c-fc14-11e8-bd87-4bb3a707bd88	b336b93e-fc14-11e8-bd87-4304c8bbc51f	system:default	4e92536e-fb72-11e8-b625-13a08229cacc	t
\.


--
-- Data for Name: schema_migrations; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.schema_migrations (version, dirty) FROM stdin;
1544764346	f
\.


--
-- Data for Name: sessions; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.sessions (id, session_id, user_id) FROM stdin;
6	\\x911744d92a9507eef48e5e5124e5a28efb3f58ecc782aecc75071da94df91129	4e92536e-fb72-11e8-b625-13a08229cacc
\.


--
-- Data for Name: topic_topics; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.topic_topics (parent_id, child_id) FROM stdin;
45dca814-e6f0-11e8-8bc1-b363da4aeace	45dcab0c-e6f0-11e8-8bc1-bbb431f062c7
df63295e-ee02-11e8-9e36-17d56b662bc8	492019e8-ee07-11e8-8642-239c847b42a7
45dcaad0-e6f0-11e8-8bc1-677f3b3c362f	45644e98-ecd9-11e8-8e0e-6fa75df8779e
45dca814-e6f0-11e8-8bc1-b363da4aeace	87240524-ee07-11e8-a945-5b5230ce136b
45dcaaf8-e6f0-11e8-8bc1-d7a04cdda708	37f6d00e-ff68-11e8-830a-63c0bad76138
df63295e-ee02-11e8-9e36-17d56b662bc8	9b022cf0-ee08-11e8-9465-5363c950bfbc
df63295e-ee02-11e8-9e36-17d56b662bc8	8df28630-ee0e-11e8-be24-cf8ad0267553
df63295e-ee02-11e8-9e36-17d56b662bc8	b7c14ab8-ee73-11e8-ba50-233c6ad09102
df63295e-ee02-11e8-9e36-17d56b662bc8	1278c44a-ee74-11e8-b558-47a668d1102a
1278c44a-ee74-11e8-b558-47a668d1102a	1bed8c36-ee74-11e8-ba50-bb71aca3cf28
b7c14ab8-ee73-11e8-ba50-233c6ad09102	6d8c25a2-ee74-11e8-a81c-f3fdddedf16f
df63295e-ee02-11e8-9e36-17d56b662bc8	48dd73b8-ee75-11e8-a81c-6bfd74718954
48dd73b8-ee75-11e8-a81c-6bfd74718954	4d4152ee-ee75-11e8-9c27-47cb09a2892a
f68eb472-ee07-11e8-aa9d-23d8a9db2265	07aa840c-ee08-11e8-ad4d-3b2ce62142ec
3c35074c-ee08-11e8-9465-338bc5df4123	99921f80-f022-11e8-8d18-a7147a02c2bc
df63295e-ee02-11e8-9e36-17d56b662bc8	2245933a-eec4-11e8-b546-a70043135c65
df63295e-ee02-11e8-9e36-17d56b662bc8	d0a36ac6-eec7-11e8-9b89-1b0756390caf
df63295e-ee02-11e8-9e36-17d56b662bc8	33b18df0-eec8-11e8-b9e7-270ae3464cf5
df63295e-ee02-11e8-9e36-17d56b662bc8	789fc094-eec8-11e8-9ff0-8f787ff5cec5
df63295e-ee02-11e8-9e36-17d56b662bc8	db093354-ef2d-11e8-a97b-73264e4fd2bf
1278c44a-ee74-11e8-b558-47a668d1102a	3e5fe9a2-ef74-11e8-9894-670527823de2
2245933a-eec4-11e8-b546-a70043135c65	c922b0ce-ee07-11e8-aa9d-131c55a142f2
f68eb472-ee07-11e8-aa9d-23d8a9db2265	5680a7a8-ee73-11e8-ba50-432dc68d001a
dae01074-eec3-11e8-9afc-4be17860c00c	e634be48-eec8-11e8-bbb3-bf16b851a67e
2245933a-eec4-11e8-b546-a70043135c65	3c35074c-ee08-11e8-9465-338bc5df4123
df63295e-ee02-11e8-9e36-17d56b662bc8	97242000-ef9a-11e8-9ace-777f6eaef6ee
97242000-ef9a-11e8-9ace-777f6eaef6ee	8c616b16-ef48-11e8-af6a-93e617bb4c16
45dca814-e6f0-11e8-8bc1-b363da4aeace	9cdd6e02-eec3-11e8-9afc-1fa8f0db9ccf
df63295e-ee02-11e8-9e36-17d56b662bc8	ce40a0c0-ef9c-11e8-9826-dbc543f3126a
8c616b16-ef48-11e8-af6a-93e617bb4c16	53680af8-ef9e-11e8-843e-737cd2a3e62f
45dcaad0-e6f0-11e8-8bc1-677f3b3c362f	b9ce6270-ef66-11e8-994e-7b6205385b8c
df63295e-ee02-11e8-9e36-17d56b662bc8	b1ae5a48-efa0-11e8-9c47-57dd8ba1d211
b7c14ab8-ee73-11e8-ba50-233c6ad09102	bd9789e0-efa2-11e8-9c47-3b8ea26f89ae
df63295e-ee02-11e8-9e36-17d56b662bc8	dae01074-eec3-11e8-9afc-4be17860c00c
bd9789e0-efa2-11e8-9c47-3b8ea26f89ae	876919a2-ef74-11e8-9bb5-8ba353cd1eac
bd9789e0-efa2-11e8-9c47-3b8ea26f89ae	c7d122e8-ef9a-11e8-9041-9331578a5d68
df63295e-ee02-11e8-9e36-17d56b662bc8	49c54b40-efa4-11e8-a66a-4f252f6e5a86
33b18df0-eec8-11e8-b9e7-270ae3464cf5	d9a643e4-ee0b-11e8-b1d2-639e5dd7d82d
f68eb472-ee07-11e8-aa9d-23d8a9db2265	f1e1b1d6-ee02-11e8-987f-5f8b10f1ae1b
33b18df0-eec8-11e8-b9e7-270ae3464cf5	a117a270-efa5-11e8-9001-07b2822d7463
a117a270-efa5-11e8-9001-07b2822d7463	7df1e544-efa5-11e8-9378-e30c0f29d98e
8c616b16-ef48-11e8-af6a-93e617bb4c16	fa96a184-efa5-11e8-9b1d-e3851e22611f
d0a36ac6-eec7-11e8-9b89-1b0756390caf	bbd64e16-efa7-11e8-9b1d-37f9635711fa
1278c44a-ee74-11e8-b558-47a668d1102a	f29b769c-efa7-11e8-ac8b-9b17246a5644
dae01074-eec3-11e8-9afc-4be17860c00c	385d490c-ff4e-11e8-8929-ff7a7358d385
e634be48-eec8-11e8-bbb3-bf16b851a67e	28f851c8-efa9-11e8-b1dc-27bf66b1795c
1278c44a-ee74-11e8-b558-47a668d1102a	28f851c8-efa9-11e8-b1dc-27bf66b1795c
df63295e-ee02-11e8-9e36-17d56b662bc8	45dca814-e6f0-11e8-8bc1-b363da4aeace
3c35074c-ee08-11e8-9465-338bc5df4123	bacbdf66-efa9-11e8-855e-2b4716af8be5
1278c44a-ee74-11e8-b558-47a668d1102a	e9819378-efa9-11e8-8ad2-bf278fa3d8f5
df63295e-ee02-11e8-9e36-17d56b662bc8	742cc830-efaa-11e8-8ad2-bbe233f83fdf
df63295e-ee02-11e8-9e36-17d56b662bc8	85c444a6-efaa-11e8-8ad2-c77d30692147
f68eb472-ee07-11e8-aa9d-23d8a9db2265	50296d5e-f9cc-11e8-92a4-0be7024158da
f1e1b1d6-ee02-11e8-987f-5f8b10f1ae1b	2e2d1642-f9cc-11e8-9d56-8f3d89524e7c
50296d5e-f9cc-11e8-92a4-0be7024158da	2e2d1642-f9cc-11e8-9d56-8f3d89524e7c
45dca814-e6f0-11e8-8bc1-b363da4aeace	45dcaad0-e6f0-11e8-8bc1-677f3b3c362f
dae01074-eec3-11e8-9afc-4be17860c00c	b4863926-eec8-11e8-93f6-e336cdf75beb
45dca814-e6f0-11e8-8bc1-b363da4aeace	0bb8bf84-f004-11e8-8f24-e74b1ee65f0f
1278c44a-ee74-11e8-b558-47a668d1102a	54ff610c-f009-11e8-a42a-7b0db2139f6c
b1ae5a48-efa0-11e8-9c47-57dd8ba1d211	bae512ee-f00b-11e8-a42a-c76c80c8c854
dae01074-eec3-11e8-9afc-4be17860c00c	bae512ee-f00b-11e8-a42a-c76c80c8c854
53680af8-ef9e-11e8-843e-737cd2a3e62f	ef6d0996-efa1-11e8-9c47-2ba4278db941
ce40a0c0-ef9c-11e8-9826-dbc543f3126a	ef6d0996-efa1-11e8-9c47-2ba4278db941
ef6d0996-efa1-11e8-9c47-2ba4278db941	05e1c34a-f382-11e8-8bc7-8f50e93b9592
df63295e-ee02-11e8-9e36-17d56b662bc8	f68eb472-ee07-11e8-aa9d-23d8a9db2265
45dca814-e6f0-11e8-8bc1-b363da4aeace	45dcaaf8-e6f0-11e8-8bc1-d7a04cdda708
df63295e-ee02-11e8-9e36-17d56b662bc8	45dcaaf8-e6f0-11e8-8bc1-d7a04cdda708
45dcaaf8-e6f0-11e8-8bc1-d7a04cdda708	6afadb04-fc1a-11e8-9367-8b3a231bc5db
\.


--
-- Data for Name: topics; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.topics (organization_id, id, name, description, created_at, updated_at, repository_id, root) FROM stdin;
b336b93e-fc14-11e8-bd87-4304c8bbc51f	97242000-ef9a-11e8-9ace-777f6eaef6ee	Travel	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	45dcaaf8-e6f0-11e8-8bc1-d7a04cdda708	Chemistry		2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	c7d122e8-ef9a-11e8-9041-9331578a5d68	World War II	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	789fc094-eec8-11e8-9ff0-8f787ff5cec5	Cities	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	e634be48-eec8-11e8-bbb3-bf16b851a67e	Photography	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	e9819378-efa9-11e8-8ad2-bf278fa3d8f5	Nepal	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	db093354-ef2d-11e8-a97b-73264e4fd2bf	GraphQL	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	8c616b16-ef48-11e8-af6a-93e617bb4c16	Transportation	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	b9ce6270-ef66-11e8-994e-7b6205385b8c	Health and nutrition	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	3e5fe9a2-ef74-11e8-9894-670527823de2	France	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	876919a2-ef74-11e8-9bb5-8ba353cd1eac	World War I	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	742cc830-efaa-11e8-8ad2-bbe233f83fdf	Mathematics	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	85c444a6-efaa-11e8-8ad2-c77d30692147	Education	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	45dca814-e6f0-11e8-8bc1-b363da4aeace	Science, mathematics and the humanities		2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	6afadb04-fc1a-11e8-9367-8b3a231bc5db	Chemistry subtopic	\N	2018-12-09 19:25:02.893268-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	dae01074-eec3-11e8-9afc-4be17860c00c	Art, design and architecture		2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	0bb8bf84-f004-11e8-8f24-e74b1ee65f0f	Economics	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	54ff610c-f009-11e8-a42a-7b0db2139f6c	United States	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	bae512ee-f00b-11e8-a42a-c76c80c8c854	Film	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	99921f80-f022-11e8-8d18-a7147a02c2bc	App Engine	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	1888bec6-f90f-11e8-9b44-bf980aad0535	http://gnusto.blog	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	05e1c34a-f382-11e8-8bc7-8f50e93b9592	Name or description	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	2e2d1642-f9cc-11e8-9d56-8f3d89524e7c	Hiring for a large business	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	50296d5e-f9cc-11e8-92a4-0be7024158da	Large businesses	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	07aa840c-ee08-11e8-ad4d-3b2ce62142ec	Amazon.com, Inc	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	3c35074c-ee08-11e8-9465-338bc5df4123	Software	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	9b022cf0-ee08-11e8-9465-5363c950bfbc	The dark web	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	d9a643e4-ee0b-11e8-b1d2-639e5dd7d82d	Natural disasters	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	8df28630-ee0e-11e8-be24-cf8ad0267553	Crime and law enforcement	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	5680a7a8-ee73-11e8-ba50-432dc68d001a	Google LLC	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	b7c14ab8-ee73-11e8-ba50-233c6ad09102	History	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	1278c44a-ee74-11e8-b558-47a668d1102a	Places	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	1bed8c36-ee74-11e8-ba50-bb71aca3cf28	Wales	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	6d8c25a2-ee74-11e8-a81c-f3fdddedf16f	The Iron Age	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	48dd73b8-ee75-11e8-a81c-6bfd74718954	Legislation	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	4d4152ee-ee75-11e8-9c27-47cb09a2892a	The Treasure Act 1996 (UK)	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	45dcab0c-e6f0-11e8-8bc1-bbb431f062c7	Physics		2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	45644e98-ecd9-11e8-8e0e-6fa75df8779e	Zoology	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	ce40a0c0-ef9c-11e8-9826-dbc543f3126a	Machines and vehicles	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	b4863926-eec8-11e8-93f6-e336cdf75beb	Architecture		2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	53680af8-ef9e-11e8-843e-737cd2a3e62f	Aviation	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	bd9789e0-efa2-11e8-9c47-3b8ea26f89ae	Wars	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	df63295e-ee02-11e8-9e36-17d56b662bc8	Everything		2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	t
b336b93e-fc14-11e8-bd87-4304c8bbc51f	37f6d00e-ff68-11e8-830a-63c0bad76138	New topicz		2018-12-14 00:19:31.536962-06	2018-12-14 00:39:05.421744-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	9c3be6be-00a1-11e9-9bfe-03433e250b67	Everything	\N	2018-12-15 13:42:52.238226-06	2018-12-15 13:42:52.238226-06	74d371f6-fc0e-11e8-b94e-2b8c1a2e2e6c	t
b336b93e-fc14-11e8-bd87-4304c8bbc51f	385d490c-ff4e-11e8-8929-ff7a7358d385	New subtopic	\N	2018-12-13 21:13:25.293111-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	f1e1b1d6-ee02-11e8-987f-5f8b10f1ae1b	Hiring for a business	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	492019e8-ee07-11e8-8642-239c847b42a7	Engineering	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	87240524-ee07-11e8-a945-5b5230ce136b	Psychology	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	f68eb472-ee07-11e8-aa9d-23d8a9db2265	Organizations, businesses and trusts	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	45dcaad0-e6f0-11e8-8bc1-677f3b3c362f	Biology		2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	2245933a-eec4-11e8-b546-a70043135c65	Computers and the Internet		2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	b1ae5a48-efa0-11e8-9c47-57dd8ba1d211	Enterntainment	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	ef6d0996-efa1-11e8-9c47-2ba4278db941	Airplanes	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	49c54b40-efa4-11e8-a66a-4f252f6e5a86	Current events	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	7df1e544-efa5-11e8-9378-e30c0f29d98e	Climate change	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	a117a270-efa5-11e8-9001-07b2822d7463	Weather	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	fa96a184-efa5-11e8-9b1d-e3851e22611f	Ships and boats	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	bbd64e16-efa7-11e8-9b1d-37f9635711fa	Space travel	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	9cdd6e02-eec3-11e8-9afc-1fa8f0db9ccf	Superconductors	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	f29b769c-efa7-11e8-ac8b-9b17246a5644	Kazakhstan	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	28f851c8-efa9-11e8-b1dc-27bf66b1795c	Landscape photography	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	bacbdf66-efa9-11e8-855e-2b4716af8be5	Golang	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	d0a36ac6-eec7-11e8-9b89-1b0756390caf	Outer space	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	33b18df0-eec8-11e8-b9e7-270ae3464cf5	Nature	\N	2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
b336b93e-fc14-11e8-bd87-4304c8bbc51f	c922b0ce-ee07-11e8-aa9d-131c55a142f2	Computer securities		2018-12-08 22:52:44.727635-06	2018-12-14 00:09:15.502897-06	b337077c-fc14-11e8-bd87-4bb3a707bd88	f
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.users (id, name, primary_email, created_at, updated_at, github_username, github_avatar_url, login) FROM stdin;
4e92536e-fb72-11e8-b625-13a08229cacc	Eric Walker	eric.walker@gmail.com	2018-12-08 23:21:39.7842-06	2018-12-09 18:44:07.08699-06	emwalker	https://avatars0.githubusercontent.com/u/760949?v=4	emwalker
\.


--
-- Name: sessions_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.sessions_id_seq', 64, true);


--
-- Name: users github_username_idx; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT github_username_idx UNIQUE (github_username);


--
-- Name: link_topics link_topics_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.link_topics
    ADD CONSTRAINT link_topics_pkey PRIMARY KEY (parent_id, child_id);


--
-- Name: links links_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.links
    ADD CONSTRAINT links_pkey PRIMARY KEY (id);


--
-- Name: links links_repository_sha1_idx; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.links
    ADD CONSTRAINT links_repository_sha1_idx UNIQUE (repository_id, sha1);


--
-- Name: organization_members organization_members_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.organization_members
    ADD CONSTRAINT organization_members_pkey PRIMARY KEY (user_id, organization_id);


--
-- Name: organizations organizations_login_idx; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.organizations
    ADD CONSTRAINT organizations_login_idx UNIQUE (login);


--
-- Name: organizations organizations_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.organizations
    ADD CONSTRAINT organizations_pkey PRIMARY KEY (id);


--
-- Name: repositories repositories_organization_name_idx; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.repositories
    ADD CONSTRAINT repositories_organization_name_idx UNIQUE (organization_id, name);


--
-- Name: repositories repositories_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.repositories
    ADD CONSTRAINT repositories_pkey PRIMARY KEY (id);


--
-- Name: schema_migrations schema_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.schema_migrations
    ADD CONSTRAINT schema_migrations_pkey PRIMARY KEY (version);


--
-- Name: sessions sessions_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.sessions
    ADD CONSTRAINT sessions_pkey PRIMARY KEY (id);


--
-- Name: sessions sessions_session_id_idx; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.sessions
    ADD CONSTRAINT sessions_session_id_idx UNIQUE (session_id);


--
-- Name: topics topics_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.topics
    ADD CONSTRAINT topics_pkey PRIMARY KEY (id);


--
-- Name: topics topics_repository_name_idx; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.topics
    ADD CONSTRAINT topics_repository_name_idx UNIQUE (name, repository_id);


--
-- Name: topic_topics topics_topics_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.topic_topics
    ADD CONSTRAINT topics_topics_pkey PRIMARY KEY (parent_id, child_id);


--
-- Name: users users_login_idx; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_login_idx UNIQUE (login);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: links_title_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX links_title_idx ON public.links USING btree (title);


--
-- Name: topics_links_child_parent_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX topics_links_child_parent_idx ON public.link_topics USING btree (child_id, parent_id);


--
-- Name: topics_name_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX topics_name_idx ON public.topics USING btree (name);


--
-- Name: topics_topics_child_parent_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX topics_topics_child_parent_idx ON public.topic_topics USING btree (child_id, parent_id);


--
-- Name: users_email_key; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX users_email_key ON public.users USING btree (primary_email);


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
-- Name: links links_repositories_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.links
    ADD CONSTRAINT links_repositories_fkey FOREIGN KEY (repository_id) REFERENCES public.repositories(id) ON DELETE CASCADE;


--
-- Name: organization_members organization_members_organization_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.organization_members
    ADD CONSTRAINT organization_members_organization_id_fkey FOREIGN KEY (organization_id) REFERENCES public.organizations(id) ON DELETE CASCADE;


--
-- Name: organization_members organization_members_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.organization_members
    ADD CONSTRAINT organization_members_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: repositories repositories_organization_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.repositories
    ADD CONSTRAINT repositories_organization_id_fkey FOREIGN KEY (organization_id) REFERENCES public.organizations(id) ON DELETE CASCADE;


--
-- Name: repositories repositories_owner_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.repositories
    ADD CONSTRAINT repositories_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: sessions sessions_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.sessions
    ADD CONSTRAINT sessions_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


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
-- Name: topics topics_repositories_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.topics
    ADD CONSTRAINT topics_repositories_fkey FOREIGN KEY (repository_id) REFERENCES public.repositories(id) ON DELETE CASCADE;


--
-- Name: topic_topics topics_topics_parent_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.topic_topics
    ADD CONSTRAINT topics_topics_parent_id_fkey FOREIGN KEY (parent_id) REFERENCES public.topics(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

