alter table user_link_reviews drop constraint user_link_reviews_pkey;
alter table user_link_reviews add column id serial;
alter table user_link_reviews add primary key (id);
