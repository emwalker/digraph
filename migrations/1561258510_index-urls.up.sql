create extension if not exists pg_trgm;

create index links_url_to_trgm_idx on links using gin (url gin_trgm_ops);
