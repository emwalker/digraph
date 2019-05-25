create table synonyms (
  id uuid primary key default uuid_generate_v1mc(),
  topic_id uuid not null references topics(id) on delete cascade,
  locale varchar(8) not null,
  name varchar(256) not null,
  created_at timestamp with time zone not null default now(),
  sort_order int not null default 1,
  unique (topic_id, locale, name)
);

create index synonyms_name_idx on synonyms (name, locale);

insert into synonyms (topic_id, name, locale)
  select t.id, t.name, 'en' from topics t
  on conflict do nothing;
