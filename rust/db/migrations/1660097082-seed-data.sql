insert into users
    (id, name, primary_email, system)
    values (
        '461c87c8-fb8f-11e8-9cbc-afde6c54d881',
        'root',
        '',
        't'
    )
    on conflict do nothing;

insert into organizations
    (id, name, login, public, owner_id, repo_prefix)
    values (
        '53900e1e-e28a-43dc-9e1d-bd0dc1cd17f3',
        'wiki',
        'wiki',
        't',
        '461c87c8-fb8f-11e8-9cbc-afde6c54d881',
        '/wiki/'
    )
    on conflict do nothing;

insert into repositories
    (id, organization_id, name, owner_id, private, prefix, root_topic_path)
    values (
        '32212616-fc1b-11e8-8eda-b70af6d8d09f',
        '53900e1e-e28a-43dc-9e1d-bd0dc1cd17f3',
        'wiki',
        '461c87c8-fb8f-11e8-9cbc-afde6c54d881',
        'f',
        '/wiki/',
        '/wiki/lBwR6Cvz4btdI23oscsp7THRytHohlol4o2IkqxcFN8'
    )
    on conflict do nothing;
