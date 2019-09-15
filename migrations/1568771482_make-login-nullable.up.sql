alter table users alter column login drop not null;

alter table users add column registered_at timestamp with time zone;
