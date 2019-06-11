create text search configuration linksdict ( copy = simple );

alter text search configuration linksdict
  alter mapping for hword, hword_part, word
  with unaccent, simple;

create index on links using gin (to_tsvector('linksdict', title));
