alter table link_topics drop column organization_id;
alter table link_topics add primary key (parent_id, child_id);
