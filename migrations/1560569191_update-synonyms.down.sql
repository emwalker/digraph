create table public.synonyms (
    id uuid default public.gen_random_uuid() not null,
    topic_id uuid not null,
    locale character varying(8) not null,
    name character varying(256) not null,
    created_at timestamp with time zone default now() not null,
    sort_order integer default 1 not null
);

alter table only public.synonyms
    add constraint synonyms_pkey primary key (id);

alter table only public.synonyms
    add constraint synonyms_topic_id_locale_name_key unique (topic_id, locale, name);

create index synonyms_name_idx on public.synonyms using btree (name, locale);

create index synonyms_to_tsvector_idx on public.synonyms using gin
  (to_tsvector('public.synonymsdict'::regconfig, (name)::text));


alter table only public.synonyms
    add constraint synonyms_topic_id_fkey foreign key (topic_id) references public.topics(id) on delete cascade;

alter table topics rename column synonyms to synonyms1;
