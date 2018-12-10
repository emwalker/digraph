alter table organizations
  drop column login,
  drop column description,
  drop column public,
  drop column system;

drop table organization_members;

update organizations set name = 'Tyrell Corporation' where name like 'General';

delete from organizations where name = 'system:default';

alter table users
  drop column login;

alter table repositories drop constraint repositories_organization_name_idx;
