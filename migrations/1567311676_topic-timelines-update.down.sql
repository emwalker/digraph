alter table topic_timeranges alter column starts_at drop not null;

alter table topic_timeranges rename to topic_timelines;
