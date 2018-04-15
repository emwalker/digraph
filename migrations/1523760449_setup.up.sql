create extension if not exists "uuid-ossp";
create extension if not exists citext;

create table organizations (
  id uuid primary key default uuid_generate_v1mc(),
  name varchar(256) not null
);

create table users (
  id uuid primary key default uuid_generate_v1mc(),
  name varchar(256) not null,
  email citext unique
);

create table topics (
  organization_id uuid not null references organizations(id) on delete cascade,
  id uuid primary key default uuid_generate_v1mc(),
  description varchar(256) not null
);
