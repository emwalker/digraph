alter table users
  add column selected_repository_id uuid,
  add constraint users_selected_repositories_fkey foreign key (selected_repository_id)
    references repositories (id) on delete cascade;
