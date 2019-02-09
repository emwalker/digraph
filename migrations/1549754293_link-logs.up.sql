create type action as enum('upsert_link', 'delete_link');

create table user_links (
  id uuid primary key default gen_random_uuid(),
  organization_id uuid not null references organizations(id) on delete cascade,
  repository_id uuid not null references repositories(id) on delete cascade,
  user_id uuid not null references users(id) on delete cascade,
  link_id uuid not null,
  created_at timestamp with time zone default now(),
  action action not null
);

create index user_links_created_at_index
  on public.user_links using btree (created_at);

create view user_link_history as
  select ul.created_at, r.name repository_name, r.id repository_id, u.name user_name,
    u.id user_id, l.url, ul.link_id, ul.action
  from user_links ul
  join repositories r on ul.repository_id = r.id
  join users u on ul.user_id = u.id
  left join links l on ul.link_id = l.id;
