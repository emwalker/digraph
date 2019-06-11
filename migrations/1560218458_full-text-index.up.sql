create extension if not exists unaccent;

create text search configuration synonymsdict ( copy = simple );

alter text search configuration synonymsdict
  alter mapping for hword, hword_part, word
  with unaccent, simple;

create index on synonyms using gin (to_tsvector('synonymsdict', name));
