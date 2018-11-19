alter table public.topics_topics add column organization_id uuid references organizations (id);
update public.topics_topics set organization_id = '45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb';
alter table public.topics_topics alter column organization_id set not null;
