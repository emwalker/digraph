drop table google_accounts;
drop table github_accounts;
alter table users drop column if exists avatar_url;
