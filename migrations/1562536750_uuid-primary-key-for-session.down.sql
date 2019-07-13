alter table sessions drop constraint sessions_pkey;
alter table sessions drop column id;

create sequence if not exists sessions_id_seq;
alter table sessions add column id int primary key not null default nextval('sessions_id_seq'::regclass);
