create table if not exists topic_timelines (
  id uuid primary key default gen_random_uuid(),
  topic_id uuid not null references topics(id) on delete cascade,
  starts_at timestamp with time zone,
  finishes_at timestamp with time zone,
  prefix_format varchar(20) not null default 'NONE'
);
