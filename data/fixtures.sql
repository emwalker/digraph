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
    sha1 character varying(40) NOT NULL
);


ALTER TABLE public.links OWNER TO postgres;

--
-- Name: organizations; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.organizations (
    id uuid DEFAULT public.uuid_generate_v1mc() NOT NULL,
    name character varying(256) NOT NULL
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
    description text
);


ALTER TABLE public.topics OWNER TO postgres;

--
-- Name: users; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.users (
    id uuid DEFAULT public.uuid_generate_v1mc() NOT NULL,
    name character varying(256) NOT NULL,
    primary_email public.citext NOT NULL
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
\.


--
-- Data for Name: links; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.links (organization_id, id, url, title, sha1) FROM stdin;
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	3e6c6fbe-ee08-11e8-bb66-77b3e2753eca	https://www.inkandswitch.com/slow-software.html	Slow Software	dc9339c15d9b66d243ac81f37cc527ed207f20f8
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	9dad3f1c-ee08-11e8-bb66-735b417eb99b	https://www.zdnet.com/article/popular-dark-web-hosting-provider-got-hacked-6500-sites-down/	Popular Dark Web hosting provider got hacked, 6,500 sites down | ZDNet	711a56a337981ae0de4929d8de1b9a9ff1534034
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	c4cda884-ee08-11e8-b1d2-db8198e90692	https://news.ycombinator.com/item?id=18504490	Popular Dark Web hosting provider hacked, 6,500 sites down | Hacker News	f24fd9588600f1ca5974061fcab97a38593c9f82
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	dde9cbd6-ee08-11e8-b1d2-572b8f794183	https://phys.org/news/2018-11-dog-cow-horse-pig-rabbit.html	The taming of the dog, cow, horse, pig and rabbit	040c320ad8cc0bdba35172df0c1316e123f880b2
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	fec9434a-eade-11e8-8231-3be3240b1542	https://github.com/	Github	d7b3438d97f335e612a566a731eea5acb8fe83c8
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	0a77ebea-ee01-11e8-86f0-5b6a2394f2e2	https://golang.org/pkg/log/	log - The Go Programming Language	51afa0ea2bd70aa9f40b2d0a65a2178786252a4e
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	10120416-eadf-11e8-8231-db1081f8c4fc	https://www.google.com/	Google	595c3cce2409a55c13076f1bac5edee529fc2e58
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	0d7fbb5a-ee07-11e8-8995-3b56ae45c0b3	https://news.ycombinator.com/item?id=18504300	My hiring experience as a submarine sonar operator in the Norwegian Navy | Hacker News	55182389cc4f963bc6e8e1821689d8e5d5b82a78
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	1dbe632c-ee07-11e8-8995-77edd669ec46	https://www.brautaset.org/articles/2018/submarine-sonar-hiring.html	My hiring experience as a submarine sonar operator in the Norwegian Navy	6b12407e657f1bf5cef7782c4300232a82a58b8f
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	4d141478-ee07-11e8-ae67-53c85dce4ba5	https://www.scientificamerican.com/article/silent-and-simple-ion-engine-powers-a-plane-with-no-moving-parts/	Silent and Simple Ion Engine Powers a Plane with No Moving Parts	e38e13d1153527faccae187642938fe47bf0a4c3
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	897061a6-ee07-11e8-a945-a32c887b43c4	https://www.cia.gov/library/center-for-the-study-of-intelligence/csi-publications/books-and-monographs/psychology-of-intelligence-analysis/art4.html	Chapter 1  — Central Intelligence Agency	777771953559d7af66cf4e4c2d213515a2658be6
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	caf89828-ee07-11e8-aa9d-a7c515f0a30c	https://techcrunch.com/2018/11/21/amazon-admits-it-exposed-customer-email-addresses-doubles-down-on-secrecy/	Amazon admits it exposed customer email addresses, but refuses to give details	dadd4fd52333e89fa9c6c5db9f435287b3ce9652
\.


--
-- Data for Name: organizations; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.organizations (id, name) FROM stdin;
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	Tyrell Corporation
\.


--
-- Data for Name: schema_migrations; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.schema_migrations (version, dirty) FROM stdin;
1542684831	f
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
\.


--
-- Data for Name: topics; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.topics (organization_id, id, name, description) FROM stdin;
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	45dcaaf8-e6f0-11e8-8bc1-d7a04cdda708	Chemistry	\N
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	df63295e-ee02-11e8-9e36-17d56b662bc8	Everything	\N
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	f1e1b1d6-ee02-11e8-987f-5f8b10f1ae1b	Hiring for a business	\N
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	492019e8-ee07-11e8-8642-239c847b42a7	Engineering	\N
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	87240524-ee07-11e8-a945-5b5230ce136b	Psychology	\N
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	c922b0ce-ee07-11e8-aa9d-131c55a142f2	Computer security	\N
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	f68eb472-ee07-11e8-aa9d-23d8a9db2265	Organizations, businesses and trusts	\N
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	07aa840c-ee08-11e8-ad4d-3b2ce62142ec	Amazon.com, Inc	\N
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	3c35074c-ee08-11e8-9465-338bc5df4123	Software	\N
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	9b022cf0-ee08-11e8-9465-5363c950bfbc	The dark web	\N
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	45dca814-e6f0-11e8-8bc1-b363da4aeace	Science	
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	45dcab0c-e6f0-11e8-8bc1-bbb431f062c7	Physics	
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	45644e98-ecd9-11e8-8e0e-6fa75df8779e	Zoology	\N
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	45dcaad0-e6f0-11e8-8bc1-677f3b3c362f	Biology	
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.users (id, name, primary_email) FROM stdin;
45dc948c-e6f0-11e8-8bc1-97e5a947cde4	Gnusto	gnusto@tyrell.test
45dca10c-e6f0-11e8-8bc1-0bba87ab695e	Frotz	frotz@tyrell.test
45dca1ac-e6f0-11e8-8bc1-7f2417a740c0	Yomin	yomin@tyrell.test
45dca1fc-e6f0-11e8-8bc1-9f412923ac8b	Bozbar	bozbar@tyrell.test
45dca260-e6f0-11e8-8bc1-a7bab2abca4f	Rezrov	rezrov@tyrell.test
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

