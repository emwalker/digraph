alter table timeranges rename to topic_timeranges;

alter table topic_timeranges add column topic_id uuid references topics(id) on delete cascade;

update topic_timeranges set topic_id = t.id
  from topics t
  where topic_timeranges.id = t.timerange_id;

alter table topic_timeranges alter column topic_id set not null;

alter table topics drop column timerange_id;
