create table public.links (
    organization_id uuid not null,
    id uuid default public.uuid_generate_v1mc() not null,
    url text not null
);

alter table public.links owner to postgres;

alter table only public.links
    add constraint links_pkey primary key (id);

alter table only public.links
    add constraint links_organization_id_fkey foreign key (organization_id)
    references public.organizations(id) on delete cascade;
