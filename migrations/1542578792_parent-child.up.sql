create table public.topics_topics (
    organization_id uuid not null references organizations (id),
    parent_id uuid not null references topics (id),
    child_id uuid not null references topics (id),
    primary key (parent_id, child_id)
);

alter table public.topics_topics owner to postgres;

create index topics_topics_child_parent_idx
    on public.topics_topics using btree (child_id, parent_id);

create table public.topics_links (
    organization_id uuid not null references organizations (id),
    parent_id uuid not null references topics (id),
    child_id uuid not null references links (id),
    primary key (organization_id, parent_id, child_id)
);

alter table public.topics_links owner to postgres;

create index topics_links_child_parent_idx
    on public.topics_links using btree (child_id, parent_id);
