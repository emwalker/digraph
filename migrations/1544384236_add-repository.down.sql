alter table links drop column repository_id;
alter table topics drop column repository_id;
drop table if exists repositories;

alter table links add constraint links_organization_sha1_idx unique (organization_id, sha1);
