create extension if not exists pgcrypto;

create table sessions (
  -- Signal that this is not a table that should be included in exports.
  id serial primary key,
  session_id bytea not null default digest(random()::text, 'sha256'),
  user_id uuid not null references users(id) on delete cascade
);

alter table sessions add constraint sessions_session_id_idx
  unique (session_id);

alter table users add column github_username varchar(256);

alter table users add constraint github_username_idx
  unique (github_username);

alter table users add column github_avatar_url varchar(1000);

alter table users drop constraint if exists users_email_key;

create index users_email_key on users (primary_email);
