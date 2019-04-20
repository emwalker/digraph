drop table if exists user_link_topics;
drop type if exists topic_action;
alter table user_links alter column created_at drop not null;
alter table user_links drop constraint if exists user_links_link_id_fkey;
