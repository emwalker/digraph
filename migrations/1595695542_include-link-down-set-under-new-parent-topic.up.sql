drop function if exists add_topic_to_topic(uuid, uuid);

create or replace function add_topic_to_topic(initial_parent_id uuid, initial_child_id uuid)
  returns void
as
$$
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
$$
language plpgsql;

begin work;
  do
  $$
  declare
    topic_id uuid;
  begin
    for topic_id in select id from topics
    loop
      perform upsert_link_down_set(topic_id);
    end loop;
  end;
  $$;
commit;
