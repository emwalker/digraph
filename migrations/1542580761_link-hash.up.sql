alter table public.links add column sha1 character varying(40);

alter table public.links add constraint links_organization_sha1_idx
    unique (organization_id, sha1);
