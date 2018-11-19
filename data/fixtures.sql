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
-- Name: topics_links; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.topics_links (
    organization_id uuid NOT NULL,
    parent_id uuid NOT NULL,
    child_id uuid NOT NULL
);


ALTER TABLE public.topics_links OWNER TO postgres;

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
-- Data for Name: links; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.links (organization_id, id, url, title, sha1) FROM stdin;
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	fec9434a-eade-11e8-8231-3be3240b1542	https://github.com/	Github	d7b3438d97f335e612a566a731eea5acb8fe83c8
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	10120416-eadf-11e8-8231-db1081f8c4fc	https://www.google.com/	Google	595c3cce2409a55c13076f1bac5edee529fc2e58
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
1542601832	f
\.


--
-- Data for Name: topic_topics; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.topic_topics (parent_id, child_id) FROM stdin;
\.


--
-- Data for Name: topics; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.topics (organization_id, id, name, description) FROM stdin;
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	45dca814-e6f0-11e8-8bc1-b363da4aeace	Science	\N
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	45dcaad0-e6f0-11e8-8bc1-677f3b3c362f	Biology	\N
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	45dcaaf8-e6f0-11e8-8bc1-d7a04cdda708	Chemistry	\N
45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb	45dcab0c-e6f0-11e8-8bc1-bbb431f062c7	Physics	\N
\.


--
-- Data for Name: topics_links; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.topics_links (organization_id, parent_id, child_id) FROM stdin;
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
-- Name: topics_links topics_links_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.topics_links
    ADD CONSTRAINT topics_links_pkey PRIMARY KEY (organization_id, parent_id, child_id);


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

CREATE INDEX topics_links_child_parent_idx ON public.topics_links USING btree (child_id, parent_id);


--
-- Name: topics_topics_child_parent_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX topics_topics_child_parent_idx ON public.topic_topics USING btree (child_id, parent_id);


--
-- Name: links links_organization_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.links
    ADD CONSTRAINT links_organization_id_fkey FOREIGN KEY (organization_id) REFERENCES public.organizations(id) ON DELETE CASCADE;


--
-- Name: topics_links topics_links_child_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.topics_links
    ADD CONSTRAINT topics_links_child_id_fkey FOREIGN KEY (child_id) REFERENCES public.links(id);


--
-- Name: topics_links topics_links_organization_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.topics_links
    ADD CONSTRAINT topics_links_organization_id_fkey FOREIGN KEY (organization_id) REFERENCES public.organizations(id);


--
-- Name: topics_links topics_links_parent_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.topics_links
    ADD CONSTRAINT topics_links_parent_id_fkey FOREIGN KEY (parent_id) REFERENCES public.topics(id);


--
-- Name: topics topics_organization_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.topics
    ADD CONSTRAINT topics_organization_id_fkey FOREIGN KEY (organization_id) REFERENCES public.organizations(id) ON DELETE CASCADE;


--
-- Name: topic_topics topics_topics_child_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.topic_topics
    ADD CONSTRAINT topics_topics_child_id_fkey FOREIGN KEY (child_id) REFERENCES public.topics(id);


--
-- Name: topic_topics topics_topics_parent_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.topic_topics
    ADD CONSTRAINT topics_topics_parent_id_fkey FOREIGN KEY (parent_id) REFERENCES public.topics(id);


--
-- PostgreSQL database dump complete
--

