create table daily_snapshot (
  created_at timestamptz not null default now(),
  topic_count int not null default 0,
  link_count int not null default 0,
  user_count int not null default 0
);
