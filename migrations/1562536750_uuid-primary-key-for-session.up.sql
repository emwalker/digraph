alter table sessions add column next_id uuid default gen_random_uuid();
alter table sessions drop column id;
alter table sessions rename column next_id to id;
alter table sessions add primary key (id);

-- alter table sessions add constraint fk_user_id foreign key (user_id) references users (id) on delete cascade;
