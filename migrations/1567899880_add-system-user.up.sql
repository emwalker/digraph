alter table users add column system boolean not null default false;

insert into users (system, name, primary_email, login)
  values (true, 'root', 'eric.walker+digraph-root@gmail.com', 'root');

update repositories set owner_id = users.id
  from users
  where repositories.name like 'General collection' and users.system;
