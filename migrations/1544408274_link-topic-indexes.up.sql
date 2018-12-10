create index links_title_idx on links (title);
alter table topics add constraint topics_repository_name_idx unique (name, repository_id);
