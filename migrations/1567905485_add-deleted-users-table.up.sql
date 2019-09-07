create table deleted_users (
  id uuid primary key default gen_random_uuid(),
  user_id uuid not null,
  deleted_at timestamp with time zone default now()
);
