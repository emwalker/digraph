alter table topic_timelines rename to topic_timeranges;

alter table topic_timeranges alter column starts_at set not null;
