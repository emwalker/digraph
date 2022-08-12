update users set avatar_url = 'https://i.pravatar.cc/460?img=32' where avatar_url is null;

alter table users alter column avatar_url set not null;
