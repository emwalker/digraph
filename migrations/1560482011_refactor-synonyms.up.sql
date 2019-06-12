alter table topics add column synonyms1 jsonb not null default '[]';

update topics t
    set synonyms1 = a.syns
from (
    select
        topic_id, jsonb_agg(jsonb_build_object('name', name, 'locale', locale)) syns
    from synonyms
    group by topic_id
) a
where a.topic_id = t.id;

update topics set synonyms1 = '[{"name":"Everything","locale":"en"}]'::jsonb where synonyms1 = '[]'::jsonb;

create index on topics using gin (synonyms1 jsonb_path_ops);
