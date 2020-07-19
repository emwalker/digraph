begin work;
  do
  $$
  declare
    topic_id uuid;
  begin
    for topic_id in select id from topics
    loop
      perform upsert_topic_down_set(topic_id);
      perform upsert_link_down_set(topic_id);
    end loop;
  end;
  $$;

  select count(0) from link_transitive_closure;
  select count(0) from topic_transitive_closure;
commit;
