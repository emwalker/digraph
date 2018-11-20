alter table topic_topics drop constraint topics_topics_child_id_fkey;
alter table topic_topics add constraint topic_topics_child_id_fkey foreign key (child_id) references topics(id) on delete cascade;
alter table topic_topics drop constraint topics_topics_parent_id_fkey;
alter table topic_topics add constraint topics_topics_parent_id_fkey foreign key (parent_id) references topics (id) on delete cascade;
alter table link_topics drop constraint link_topics_parent_id_fkey;
alter table link_topics add constraint link_topics_parent_id_fkey foreign key (parent_id) references topics(id) on delete cascade;
alter table link_topics drop constraint link_topics_child_id_fkey;
alter table link_topics add constraint link_topics_child_id_fkey foreign key (child_id) references links(id) on delete cascade;
