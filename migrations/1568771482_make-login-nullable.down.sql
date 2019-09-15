alter table users drop column if exists registered_at;

alter table users alter column login set not null;
