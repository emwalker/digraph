create table user_link_reviews (
  id serial primary key,
  user_id uuid not null references users(id) on delete cascade,
  link_id uuid not null references links(id) on delete cascade,
  created_at timestamp with time zone not null default now(),
  updated_at timestamp with time zone not null default now(),
  reviewed_at timestamp with time zone,
  unique (user_id, link_id)
);

create index user_link_reviews_user_idx on user_link_reviews (user_id, reviewed_at);
