create table link_transitive_closure (
  id serial primary key,
  parent_id uuid not null references topics(id) on delete cascade,
  child_id uuid not null references links(id) on delete cascade
);

alter table link_transitive_closure
  add constraint link_transitive_closure_idx unique (parent_id, child_id);

create table topic_transitive_closure (
  id serial primary key,
  parent_id uuid not null references topics(id) on delete cascade,
  child_id uuid not null references topics(id) on delete cascade
);

alter table topic_transitive_closure
  add constraint topic_transitive_closure_idx unique (parent_id, child_id);

-- Return all of the subtopics of this topic, including the topic itself.
create or replace function topic_down_set(topic_id uuid)
  returns table (parent_id uuid, child_id uuid)
as
$$
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
$$
language plpgsql;

-- Return all of the links associated with subtopics of this topic, as well as links associated with the
-- topic itself.
create or replace function link_down_set(topic_id uuid)
  returns table (parent_id uuid, child_id uuid)
as
$$
begin
  return query
  select topic_id, lt.child_id
  from topic_down_set(topic_id) ct
  inner join link_topics lt on lt.parent_id = ct.child_id;
end;
$$
language plpgsql;

create or replace function upsert_link_down_set(topic_id uuid) returns void as
$$
begin
  insert into link_transitive_closure (parent_id, child_id)
    select * from link_down_set(topic_id)
    on conflict do nothing;
end;
$$
language plpgsql;

create or replace function upsert_topic_down_set(topic_id uuid) returns void as
$$
begin
  insert into topic_transitive_closure (parent_id, child_id)
    select topic_id, child_id from topic_down_set(topic_id)
    on conflict do nothing;
end;
$$
language plpgsql;

-- Return all of the subtopics of this topic, including the topic itself.
create or replace function topic_upper_set(topic_id uuid)
  returns table (parent_id uuid, child_id uuid)
as
$$
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
$$
language plpgsql;

create or replace function add_topic_to_topic(parent_id uuid, initial_child_id uuid)
  returns void
as
$$
begin
  insert into topic_topics (parent_id, child_id)
    values (parent_id, initial_child_id)
    on conflict do nothing;
  insert into topic_transitive_closure (parent_id, child_id)
    select us.parent_id, initial_child_id
    from topic_upper_set(parent_id) us
    on conflict do nothing;
end;
$$
language plpgsql;

create or replace function add_topic_to_link(topic_id uuid, link_id uuid)
  returns void
as
$$
begin
  insert into link_topics (parent_id, child_id)
    values (topic_id, link_id)
    on conflict do nothing;
  insert into link_transitive_closure (parent_id, child_id)
    select us.parent_id, link_id
    from topic_upper_set(topic_id) us
    on conflict do nothing;
end;
$$
language plpgsql;
