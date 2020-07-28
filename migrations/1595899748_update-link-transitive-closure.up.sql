begin work;
  delete from link_transitive_closure;

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
