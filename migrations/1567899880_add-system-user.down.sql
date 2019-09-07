update repositories set owner_id = '461c87c8-fb8f-11e8-9cbc-afde6c54d881';

delete from users where name like 'root' and system;

alter table users drop column system;
