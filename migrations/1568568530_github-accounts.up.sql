create table github_accounts (
  id uuid primary key default gen_random_uuid(),
  user_id uuid not null references users(id) on delete cascade,
  username character varying(256) not null,
  name character varying(256) not null,
  primary_email citext not null,
  avatar_url character varying(1000) not null
);

create table google_accounts (
  id uuid primary key default gen_random_uuid(),
  user_id uuid not null references users(id) on delete cascade,
  profile_id character varying(256) not null,
  name character varying(256) not null,
  primary_email citext not null,
  avatar_url character varying(1000) not null
);

insert into github_accounts (user_id, username, name, primary_email, avatar_url)
  select id, github_username, name, primary_email, github_avatar_url
  from users
  where github_username is not null;

alter table users add column avatar_url varchar(256);

update users set avatar_url = github_avatar_url;
