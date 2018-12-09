create table repositories (
  id uuid primary key default uuid_generate_v1mc(),
  organization_id uuid not null references organizations(id) on delete cascade,
  name varchar(256) not null,
  owner_id uuid not null references users(id) on delete cascade,
  system boolean not null default false
);

insert into repositories (name, system, organization_id, owner_id)
  select 'system:default', true, '45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb', u.id
  from users u where name like 'Eric Walker';

alter table links
  add column repository_id uuid,
  add constraint links_repositories_fkey foreign key (repository_id) references repositories (id)
    on delete cascade;

alter table topics
  add column repository_id uuid,
  add constraint topics_repositories_fkey foreign key (repository_id) references repositories (id)
    on delete cascade;

update links
  set repository_id = r.id
  from repositories r;

update topics
  set repository_id = r.id
  from repositories r;

alter table links alter column repository_id set not null;
alter table topics alter column repository_id set not null;

alter table links drop constraint links_organization_sha1_idx;
alter table links add constraint links_repository_sha1_idx unique (repository_id, sha1);
