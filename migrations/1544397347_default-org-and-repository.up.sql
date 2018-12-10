delete from users where github_username = '' or github_username is null;

alter table users
  add column login varchar(256) not null default '';

update users set login = github_username;

alter table users
  add constraint users_login_idx unique (login);

alter table users alter column login drop default;

alter table organizations
  add column login varchar(256),
  add column description varchar(256),
  add column public boolean not null default false,
  add column system boolean not null default false;

create table organization_members (
  organization_id uuid not null references organizations(id) on delete cascade,
  user_id uuid not null references users(id) on delete cascade,
  owner boolean not null default false,
  primary key (user_id, organization_id)
);

update organizations
  set
    login = 'wiki',
    name = 'General',
    description = 'The default organization when an organization is not provided',
    public = true
  where name like 'Tyrell Corporation' or name like 'General';

alter table organizations
  alter column login set not null,
  add constraint organizations_login_idx unique (login);

insert into organizations (login, name, description, public, system)
  select
    'emwalker', 'system:default', '', false, true;

insert into organization_members (organization_id, user_id, owner)
  select o.id, u.id, true
  from organizations o
  cross join users u
  where o.name like 'General' or o.login like 'emwalker'
    and u.login like 'emwalker';

alter table repositories
  add constraint repositories_organization_name_idx unique (organization_id, name);

insert into repositories (organization_id, name, owner_id, system)
  select o.id, 'system:default', u.id, true
  from organizations o
  cross join users u
  where o.login like 'emwalker' and u.login like 'emwalker';
