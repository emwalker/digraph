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
