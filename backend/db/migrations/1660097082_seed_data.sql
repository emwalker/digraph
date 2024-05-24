insert into users
    (id, name, primary_email, system, avatar_url)
    values (
        '461c87c8-fb8f-11e8-9cbc-afde6c54d881',
        'root',
        '',
        't',
        'https://i.pravatar.cc/460?img=32'
    )
    on conflict do nothing;

insert into organizations
    (id, name, login, public, owner_id)
    values (
        '53900e1e-e28a-43dc-9e1d-bd0dc1cd17f3',
        'wiki',
        'wiki',
        't',
        '461c87c8-fb8f-11e8-9cbc-afde6c54d881'
    )
    on conflict do nothing;

insert into repositories
    (id, organization_id, name, owner_id, private)
    values (
        '32212616-fc1b-11e8-8eda-b70af6d8d09f',
        '53900e1e-e28a-43dc-9e1d-bd0dc1cd17f3',
        'wiki',
        '461c87c8-fb8f-11e8-9cbc-afde6c54d881',
        'f'
    )
    on conflict do nothing;
