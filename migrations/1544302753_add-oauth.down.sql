drop table if exists sessions;
drop extension if exists pgcrypto;
drop index if exists sessions_session_id_idx;
alter table users drop column github_username;
alter table users drop column github_avatar_url;
alter table users drop column if exists avatar_url;

drop index if exists users_email_key;
alter table users add constraint users_email_key unique (primary_email);
