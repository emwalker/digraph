alter table topics add column timerange_id uuid references topic_timeranges(id) on delete set null;

update topics set timerange_id = tr.id
  from topic_timeranges tr
  where topics.id = tr.topic_id;

alter table topic_timeranges rename to timeranges;

alter table timeranges drop column topic_id;
