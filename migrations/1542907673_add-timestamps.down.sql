drop trigger if exists links_updated_at on links;
drop trigger if exists organizations_updated_at on organizations;
drop trigger if exists topics_updated_at on topics;
drop trigger if exists users_updated_at on users;

drop extension moddatetime;

alter table links drop column created_at;
alter table links drop column updated_at;
alter table organizations drop column created_at;
alter table organizations drop column updated_at;
alter table topics drop column created_at;
alter table topics drop column updated_at;
alter table users drop column created_at;
alter table users drop column updated_at;
