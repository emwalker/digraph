alter table user_link_reviews drop constraint user_link_reviews_pkey;
alter table user_link_reviews drop column id;
alter table user_link_reviews add primary key (user_id, link_id);
