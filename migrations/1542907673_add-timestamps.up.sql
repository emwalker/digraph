create extension moddatetime;

alter table links add column created_at timestamptz not null default now();
alter table links add column updated_at timestamptz not null default now();
alter table organizations add column created_at timestamptz not null default now();
alter table organizations add column updated_at timestamptz not null default now();
alter table topics add column created_at timestamptz not null default now();
alter table topics add column updated_at timestamptz not null default now();
alter table users add column created_at timestamptz not null default now();
alter table users add column updated_at timestamptz not null default now();

create trigger links_updated_at
  before update on links
  for each row
  execute procedure moddatetime (updated_at);

create trigger organizations_updated_at
  before update on organizations
  for each row
  execute procedure moddatetime (updated_at);

create trigger topics_updated_at
  before update on topics
  for each row
  execute procedure moddatetime (updated_at);

create trigger users_updated_at
  before update on users
  for each row
  execute procedure moddatetime (updated_at);
