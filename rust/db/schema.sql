--
-- PostgreSQL database dump
--

-- Dumped from database version 13.6 (Ubuntu 13.6-0ubuntu0.21.10.1)
-- Dumped by pg_dump version 13.6 (Ubuntu 13.6-0ubuntu0.21.10.1)

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

--
-- Name: citext; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS citext WITH SCHEMA public;


--
-- Name: EXTENSION citext; Type: COMMENT; Schema: -; Owner:
--

COMMENT ON EXTENSION citext IS 'data type for case-insensitive character strings';


--
-- Name: pg_trgm; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS pg_trgm WITH SCHEMA public;


--
-- Name: EXTENSION pg_trgm; Type: COMMENT; Schema: -; Owner:
--

COMMENT ON EXTENSION pg_trgm IS 'text similarity measurement and index searching based on trigrams';


--
-- Name: pgcrypto; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS pgcrypto WITH SCHEMA public;


--
-- Name: EXTENSION pgcrypto; Type: COMMENT; Schema: -; Owner:
--

COMMENT ON EXTENSION pgcrypto IS 'cryptographic functions';


--
-- Name: unaccent; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS unaccent WITH SCHEMA public;


--
-- Name: EXTENSION unaccent; Type: COMMENT; Schema: -; Owner:
--

COMMENT ON EXTENSION unaccent IS 'text search dictionary that removes accents';


--
-- Name: action; Type: TYPE; Schema: public; Owner: postgres
--

CREATE TYPE public.action AS ENUM (
    'upsert_link',
    'delete_link'
);


ALTER TYPE public.action OWNER TO postgres;

--
-- Name: topic_action; Type: TYPE; Schema: public; Owner: postgres
--

CREATE TYPE public.topic_action AS ENUM (
    'topic_added',
    'topic_removed'
);


ALTER TYPE public.topic_action OWNER TO postgres;

--
-- Name: add_topic_to_link(uuid, uuid); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.add_topic_to_link(topic_id uuid, link_id uuid) RETURNS void
    LANGUAGE plpgsql
    AS $$
begin
  insert into link_topics (parent_id, child_id)
    values (topic_id, link_id)
    on conflict do nothing;
  insert into link_transitive_closure (parent_id, child_id)
    select us.parent_id, link_id
    from topic_upper_set(topic_id) us
    on conflict do nothing;
end;
$$;


ALTER FUNCTION public.add_topic_to_link(topic_id uuid, link_id uuid) OWNER TO postgres;

--
-- Name: add_topic_to_topic(uuid, uuid); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.add_topic_to_topic(initial_parent_id uuid, initial_child_id uuid) RETURNS void
    LANGUAGE plpgsql
    AS $$
begin
  -- Add the new relationship
  insert into topic_topics (parent_id, child_id)
    values (initial_parent_id, initial_child_id)
    on conflict do nothing;
  -- Update the topic upward set of the child topic
  insert into topic_transitive_closure (parent_id, child_id)
    select us.parent_id, us.child_id
    from topic_upper_set(initial_child_id) us
    on conflict do nothing;
  -- Add the link down set of the child topic to the new parent topic
  insert into link_transitive_closure (parent_id, child_id)
    select initial_parent_id, ds.child_id
    from link_down_set(initial_child_id) ds
    on conflict do nothing;
end;
$$;


ALTER FUNCTION public.add_topic_to_topic(initial_parent_id uuid, initial_child_id uuid) OWNER TO postgres;

--
-- Name: link_down_set(uuid); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.link_down_set(topic_id uuid) RETURNS TABLE(parent_id uuid, child_id uuid)
    LANGUAGE plpgsql
    AS $$
begin
  return query
  select topic_id, lt.child_id
  from topic_down_set(topic_id) ct
  inner join link_topics lt on lt.parent_id = ct.child_id;
end;
$$;


ALTER FUNCTION public.link_down_set(topic_id uuid) OWNER TO postgres;

--
-- Name: topic_down_set(uuid); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.topic_down_set(topic_id uuid) RETURNS TABLE(parent_id uuid, child_id uuid)
    LANGUAGE plpgsql
    AS $$
begin
  return query
  with recursive
  child_topics as (
    select topic_id as parent_id, topic_id as child_id
  union
    select pt.child_id, ct.child_id
    from topic_topics ct
    inner join child_topics pt on pt.child_id = ct.parent_id
  )
  select topic_id, ct.child_id from child_topics ct;
end;
$$;


ALTER FUNCTION public.topic_down_set(topic_id uuid) OWNER TO postgres;

--
-- Name: topic_upper_set(uuid); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.topic_upper_set(topic_id uuid) RETURNS TABLE(parent_id uuid, child_id uuid)
    LANGUAGE plpgsql
    AS $$
begin
  return query
  with recursive
  parent_topics as (
    select topic_id as parent_id, topic_id as child_id
  union
    select pt.parent_id, ct.child_id
    from topic_topics pt
    inner join parent_topics ct on pt.child_id = ct.parent_id
  )
  select pt.parent_id, topic_id from parent_topics pt;
end;
$$;


ALTER FUNCTION public.topic_upper_set(topic_id uuid) OWNER TO postgres;

--
-- Name: upsert_link_down_set(uuid); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.upsert_link_down_set(topic_id uuid) RETURNS void
    LANGUAGE plpgsql
    AS $$
begin
  insert into link_transitive_closure (parent_id, child_id)
    select * from link_down_set(topic_id)
    on conflict do nothing;
end;
$$;


ALTER FUNCTION public.upsert_link_down_set(topic_id uuid) OWNER TO postgres;

--
-- Name: upsert_topic_down_set(uuid); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.upsert_topic_down_set(topic_id uuid) RETURNS void
    LANGUAGE plpgsql
    AS $$
begin
  insert into topic_transitive_closure (parent_id, child_id)
    select topic_id, child_id from topic_down_set(topic_id)
    on conflict do nothing;
end;
$$;


ALTER FUNCTION public.upsert_topic_down_set(topic_id uuid) OWNER TO postgres;

--
-- Name: linksdict; Type: TEXT SEARCH CONFIGURATION; Schema: public; Owner: postgres
--

CREATE TEXT SEARCH CONFIGURATION public.linksdict (
    PARSER = pg_catalog."default" );

ALTER TEXT SEARCH CONFIGURATION public.linksdict
    ADD MAPPING FOR asciiword WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.linksdict
    ADD MAPPING FOR word WITH public.unaccent, simple;

ALTER TEXT SEARCH CONFIGURATION public.linksdict
    ADD MAPPING FOR numword WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.linksdict
    ADD MAPPING FOR email WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.linksdict
    ADD MAPPING FOR url WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.linksdict
    ADD MAPPING FOR host WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.linksdict
    ADD MAPPING FOR sfloat WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.linksdict
    ADD MAPPING FOR version WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.linksdict
    ADD MAPPING FOR hword_numpart WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.linksdict
    ADD MAPPING FOR hword_part WITH public.unaccent, simple;

ALTER TEXT SEARCH CONFIGURATION public.linksdict
    ADD MAPPING FOR hword_asciipart WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.linksdict
    ADD MAPPING FOR numhword WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.linksdict
    ADD MAPPING FOR asciihword WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.linksdict
    ADD MAPPING FOR hword WITH public.unaccent, simple;

ALTER TEXT SEARCH CONFIGURATION public.linksdict
    ADD MAPPING FOR url_path WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.linksdict
    ADD MAPPING FOR file WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.linksdict
    ADD MAPPING FOR "float" WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.linksdict
    ADD MAPPING FOR "int" WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.linksdict
    ADD MAPPING FOR uint WITH simple;


ALTER TEXT SEARCH CONFIGURATION public.linksdict OWNER TO postgres;

--
-- Name: synonymsdict; Type: TEXT SEARCH CONFIGURATION; Schema: public; Owner: postgres
--

CREATE TEXT SEARCH CONFIGURATION public.synonymsdict (
    PARSER = pg_catalog."default" );

ALTER TEXT SEARCH CONFIGURATION public.synonymsdict
    ADD MAPPING FOR asciiword WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.synonymsdict
    ADD MAPPING FOR word WITH public.unaccent, simple;

ALTER TEXT SEARCH CONFIGURATION public.synonymsdict
    ADD MAPPING FOR numword WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.synonymsdict
    ADD MAPPING FOR email WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.synonymsdict
    ADD MAPPING FOR url WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.synonymsdict
    ADD MAPPING FOR host WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.synonymsdict
    ADD MAPPING FOR sfloat WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.synonymsdict
    ADD MAPPING FOR version WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.synonymsdict
    ADD MAPPING FOR hword_numpart WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.synonymsdict
    ADD MAPPING FOR hword_part WITH public.unaccent, simple;

ALTER TEXT SEARCH CONFIGURATION public.synonymsdict
    ADD MAPPING FOR hword_asciipart WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.synonymsdict
    ADD MAPPING FOR numhword WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.synonymsdict
    ADD MAPPING FOR asciihword WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.synonymsdict
    ADD MAPPING FOR hword WITH public.unaccent, simple;

ALTER TEXT SEARCH CONFIGURATION public.synonymsdict
    ADD MAPPING FOR url_path WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.synonymsdict
    ADD MAPPING FOR file WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.synonymsdict
    ADD MAPPING FOR "float" WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.synonymsdict
    ADD MAPPING FOR "int" WITH simple;

ALTER TEXT SEARCH CONFIGURATION public.synonymsdict
    ADD MAPPING FOR uint WITH simple;


ALTER TEXT SEARCH CONFIGURATION public.synonymsdict OWNER TO postgres;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: _sqlx_migrations; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public._sqlx_migrations (
    version bigint NOT NULL,
    description text NOT NULL,
    installed_on timestamp with time zone DEFAULT now() NOT NULL,
    success boolean NOT NULL,
    checksum bytea NOT NULL,
    execution_time bigint NOT NULL
);


ALTER TABLE public._sqlx_migrations OWNER TO postgres;

--
-- Name: daily_snapshot; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.daily_snapshot (
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    topic_count integer DEFAULT 0 NOT NULL,
    link_count integer DEFAULT 0 NOT NULL,
    user_count integer DEFAULT 0 NOT NULL,
    active_user_count integer DEFAULT 0 NOT NULL
);


ALTER TABLE public.daily_snapshot OWNER TO postgres;

--
-- Name: deleted_users; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.deleted_users (
    id uuid DEFAULT public.gen_random_uuid() NOT NULL,
    user_id uuid NOT NULL,
    deleted_at timestamp with time zone DEFAULT now()
);


ALTER TABLE public.deleted_users OWNER TO postgres;

--
-- Name: github_accounts; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.github_accounts (
    id uuid DEFAULT public.gen_random_uuid() NOT NULL,
    user_id uuid NOT NULL,
    username character varying(256) NOT NULL,
    name character varying(256) NOT NULL,
    primary_email public.citext NOT NULL,
    avatar_url character varying(1000) NOT NULL
);


ALTER TABLE public.github_accounts OWNER TO postgres;

--
-- Name: google_accounts; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.google_accounts (
    id uuid DEFAULT public.gen_random_uuid() NOT NULL,
    user_id uuid NOT NULL,
    profile_id character varying(256) NOT NULL,
    name character varying(256) NOT NULL,
    primary_email public.citext NOT NULL,
    avatar_url character varying(1000) NOT NULL
);


ALTER TABLE public.google_accounts OWNER TO postgres;

--
-- Name: link_topics; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.link_topics (
    parent_id uuid NOT NULL,
    child_id uuid NOT NULL
);


ALTER TABLE public.link_topics OWNER TO postgres;

--
-- Name: link_transitive_closure; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.link_transitive_closure (
    id integer NOT NULL,
    parent_id uuid NOT NULL,
    child_id uuid NOT NULL
);


ALTER TABLE public.link_transitive_closure OWNER TO postgres;

--
-- Name: link_transitive_closure_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.link_transitive_closure_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.link_transitive_closure_id_seq OWNER TO postgres;

--
-- Name: link_transitive_closure_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.link_transitive_closure_id_seq OWNED BY public.link_transitive_closure.id;


--
-- Name: links; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.links (
    organization_id uuid NOT NULL,
    id uuid DEFAULT public.gen_random_uuid() NOT NULL,
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
    id uuid DEFAULT public.gen_random_uuid() NOT NULL,
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
    id uuid DEFAULT public.gen_random_uuid() NOT NULL,
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
    session_id bytea DEFAULT public.digest((random())::text, 'sha256'::text) NOT NULL,
    user_id uuid NOT NULL,
    id uuid DEFAULT public.gen_random_uuid() NOT NULL
);


ALTER TABLE public.sessions OWNER TO postgres;

--
-- Name: timeranges; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.timeranges (
    id uuid DEFAULT public.gen_random_uuid() NOT NULL,
    starts_at timestamp with time zone NOT NULL,
    finishes_at timestamp with time zone,
    prefix_format character varying(20) DEFAULT 'NONE'::character varying NOT NULL
);


ALTER TABLE public.timeranges OWNER TO postgres;

--
-- Name: topic_topics; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.topic_topics (
    parent_id uuid NOT NULL,
    child_id uuid NOT NULL
);


ALTER TABLE public.topic_topics OWNER TO postgres;

--
-- Name: topic_transitive_closure; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.topic_transitive_closure (
    id integer NOT NULL,
    parent_id uuid NOT NULL,
    child_id uuid NOT NULL
);


ALTER TABLE public.topic_transitive_closure OWNER TO postgres;

--
-- Name: topic_transitive_closure_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.topic_transitive_closure_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.topic_transitive_closure_id_seq OWNER TO postgres;

--
-- Name: topic_transitive_closure_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.topic_transitive_closure_id_seq OWNED BY public.topic_transitive_closure.id;


--
-- Name: topics; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.topics (
    organization_id uuid NOT NULL,
    id uuid DEFAULT public.gen_random_uuid() NOT NULL,
    name character varying(256) NOT NULL,
    description text,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    repository_id uuid NOT NULL,
    root boolean DEFAULT false NOT NULL,
    synonyms jsonb DEFAULT '[]'::jsonb NOT NULL,
    timerange_id uuid
);


ALTER TABLE public.topics OWNER TO postgres;

--
-- Name: user_links; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.user_links (
    id uuid DEFAULT public.gen_random_uuid() NOT NULL,
    organization_id uuid NOT NULL,
    repository_id uuid NOT NULL,
    user_id uuid NOT NULL,
    link_id uuid NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    action public.action NOT NULL
);


ALTER TABLE public.user_links OWNER TO postgres;

--
-- Name: users; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.users (
    id uuid DEFAULT public.gen_random_uuid() NOT NULL,
    name character varying(256) NOT NULL,
    primary_email public.citext NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    github_username character varying(256),
    github_avatar_url character varying(1000),
    login character varying(256),
    selected_repository_id uuid,
    system boolean DEFAULT false NOT NULL,
    avatar_url character varying(256),
    registered_at timestamp with time zone
);


ALTER TABLE public.users OWNER TO postgres;

--
-- Name: user_link_history; Type: VIEW; Schema: public; Owner: postgres
--

CREATE VIEW public.user_link_history AS
 SELECT ul.created_at,
    r.name AS repository_name,
    r.id AS repository_id,
    u.name AS user_name,
    u.id AS user_id,
    l.url,
    ul.link_id,
    ul.action
   FROM (((public.user_links ul
     JOIN public.repositories r ON ((ul.repository_id = r.id)))
     JOIN public.users u ON ((ul.user_id = u.id)))
     LEFT JOIN public.links l ON ((ul.link_id = l.id)));


ALTER TABLE public.user_link_history OWNER TO postgres;

--
-- Name: user_link_reviews; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.user_link_reviews (
    user_id uuid NOT NULL,
    link_id uuid NOT NULL,
    created_at timestamp with time zone DEFAULT now() NOT NULL,
    updated_at timestamp with time zone DEFAULT now() NOT NULL,
    reviewed_at timestamp with time zone
);


ALTER TABLE public.user_link_reviews OWNER TO postgres;

--
-- Name: user_link_topics; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.user_link_topics (
    id uuid DEFAULT public.gen_random_uuid() NOT NULL,
    user_link_id uuid NOT NULL,
    action public.topic_action NOT NULL,
    topic_id uuid NOT NULL
);


ALTER TABLE public.user_link_topics OWNER TO postgres;

--
-- Name: link_transitive_closure id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.link_transitive_closure ALTER COLUMN id SET DEFAULT nextval('public.link_transitive_closure_id_seq'::regclass);


--
-- Name: topic_transitive_closure id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.topic_transitive_closure ALTER COLUMN id SET DEFAULT nextval('public.topic_transitive_closure_id_seq'::regclass);


--
-- Name: _sqlx_migrations _sqlx_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public._sqlx_migrations
    ADD CONSTRAINT _sqlx_migrations_pkey PRIMARY KEY (version);


--
-- Name: users github_username_idx; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT github_username_idx UNIQUE (github_username);


--
-- Name: google_accounts google_accounts_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.google_accounts
    ADD CONSTRAINT google_accounts_pkey PRIMARY KEY (id);


--
-- Name: link_topics link_topics_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.link_topics
    ADD CONSTRAINT link_topics_pkey PRIMARY KEY (parent_id, child_id);


--
-- Name: link_transitive_closure link_transitive_closure_idx; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.link_transitive_closure
    ADD CONSTRAINT link_transitive_closure_idx UNIQUE (parent_id, child_id);


--
-- Name: link_transitive_closure link_transitive_closure_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.link_transitive_closure
    ADD CONSTRAINT link_transitive_closure_pkey PRIMARY KEY (id);


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
-- Name: timeranges topic_timelines_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.timeranges
    ADD CONSTRAINT topic_timelines_pkey PRIMARY KEY (id);


--
-- Name: topic_transitive_closure topic_transitive_closure_idx; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.topic_transitive_closure
    ADD CONSTRAINT topic_transitive_closure_idx UNIQUE (parent_id, child_id);


--
-- Name: topic_transitive_closure topic_transitive_closure_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.topic_transitive_closure
    ADD CONSTRAINT topic_transitive_closure_pkey PRIMARY KEY (id);


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
-- Name: user_link_reviews user_link_reviews_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_link_reviews
    ADD CONSTRAINT user_link_reviews_pkey PRIMARY KEY (user_id, link_id);


--
-- Name: user_link_reviews user_link_reviews_user_id_link_id_key; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_link_reviews
    ADD CONSTRAINT user_link_reviews_user_id_link_id_key UNIQUE (user_id, link_id);


--
-- Name: user_link_topics user_link_topics_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_link_topics
    ADD CONSTRAINT user_link_topics_pkey PRIMARY KEY (id);


--
-- Name: user_links user_links_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_links
    ADD CONSTRAINT user_links_pkey PRIMARY KEY (id);


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
-- Name: links_to_tsvector_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX links_to_tsvector_idx ON public.links USING gin (to_tsvector('public.linksdict'::regconfig, title));


--
-- Name: links_url_to_trgm_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX links_url_to_trgm_idx ON public.links USING gin (url public.gin_trgm_ops);


--
-- Name: topics_links_child_parent_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX topics_links_child_parent_idx ON public.link_topics USING btree (child_id, parent_id);


--
-- Name: topics_synonyms1_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX topics_synonyms1_idx ON public.topics USING gin (synonyms jsonb_path_ops);


--
-- Name: topics_topics_child_parent_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX topics_topics_child_parent_idx ON public.topic_topics USING btree (child_id, parent_id);


--
-- Name: user_link_reviews_user_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX user_link_reviews_user_idx ON public.user_link_reviews USING btree (user_id, reviewed_at);


--
-- Name: user_links_created_at_index; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX user_links_created_at_index ON public.user_links USING btree (created_at);


--
-- Name: users_email_key; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX users_email_key ON public.users USING btree (primary_email);


--
-- Name: github_accounts github_accounts_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.github_accounts
    ADD CONSTRAINT github_accounts_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: google_accounts google_accounts_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.google_accounts
    ADD CONSTRAINT google_accounts_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


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
-- Name: link_transitive_closure link_transitive_closure_child_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.link_transitive_closure
    ADD CONSTRAINT link_transitive_closure_child_id_fkey FOREIGN KEY (child_id) REFERENCES public.links(id) ON DELETE CASCADE;


--
-- Name: link_transitive_closure link_transitive_closure_parent_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.link_transitive_closure
    ADD CONSTRAINT link_transitive_closure_parent_id_fkey FOREIGN KEY (parent_id) REFERENCES public.topics(id) ON DELETE CASCADE;


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
-- Name: topic_transitive_closure topic_transitive_closure_child_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.topic_transitive_closure
    ADD CONSTRAINT topic_transitive_closure_child_id_fkey FOREIGN KEY (child_id) REFERENCES public.topics(id) ON DELETE CASCADE;


--
-- Name: topic_transitive_closure topic_transitive_closure_parent_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.topic_transitive_closure
    ADD CONSTRAINT topic_transitive_closure_parent_id_fkey FOREIGN KEY (parent_id) REFERENCES public.topics(id) ON DELETE CASCADE;


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
-- Name: topics topics_timerange_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.topics
    ADD CONSTRAINT topics_timerange_id_fkey FOREIGN KEY (timerange_id) REFERENCES public.timeranges(id) ON DELETE SET NULL;


--
-- Name: topic_topics topics_topics_parent_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.topic_topics
    ADD CONSTRAINT topics_topics_parent_id_fkey FOREIGN KEY (parent_id) REFERENCES public.topics(id) ON DELETE CASCADE;


--
-- Name: user_link_reviews user_link_reviews_link_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_link_reviews
    ADD CONSTRAINT user_link_reviews_link_id_fkey FOREIGN KEY (link_id) REFERENCES public.links(id) ON DELETE CASCADE;


--
-- Name: user_link_reviews user_link_reviews_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_link_reviews
    ADD CONSTRAINT user_link_reviews_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: user_link_topics user_link_topics_topic_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_link_topics
    ADD CONSTRAINT user_link_topics_topic_id_fkey FOREIGN KEY (topic_id) REFERENCES public.topics(id) ON DELETE CASCADE;


--
-- Name: user_link_topics user_link_topics_user_link_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_link_topics
    ADD CONSTRAINT user_link_topics_user_link_id_fkey FOREIGN KEY (user_link_id) REFERENCES public.user_links(id) ON DELETE CASCADE;


--
-- Name: user_links user_links_link_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_links
    ADD CONSTRAINT user_links_link_id_fkey FOREIGN KEY (link_id) REFERENCES public.links(id) ON DELETE CASCADE;


--
-- Name: user_links user_links_organization_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_links
    ADD CONSTRAINT user_links_organization_id_fkey FOREIGN KEY (organization_id) REFERENCES public.organizations(id) ON DELETE CASCADE;


--
-- Name: user_links user_links_repository_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_links
    ADD CONSTRAINT user_links_repository_id_fkey FOREIGN KEY (repository_id) REFERENCES public.repositories(id) ON DELETE CASCADE;


--
-- Name: user_links user_links_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.user_links
    ADD CONSTRAINT user_links_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;


--
-- Name: users users_selected_repositories_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_selected_repositories_fkey FOREIGN KEY (selected_repository_id) REFERENCES public.repositories(id) ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

