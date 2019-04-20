alter table user_links alter column created_at set not null;

create type topic_action as enum ('topic_added', 'topic_removed');

create table user_link_topics (
  id uuid primary key default uuid_generate_v1mc(),
  user_link_id uuid not null references user_links(id) on delete cascade,
  action topic_action not null,
  topic_id uuid not null references topics(id) on delete cascade
);

delete from user_links
  using user_links ul
  left join links l on ul.link_id = l.id
  where user_links.id = ul.id and l.id is null;

alter table user_links
  add constraint user_links_link_id_fkey
  foreign key (link_id)
  references links(id)
  on delete cascade;
