do $$
declare lastid uuid;
begin
  insert into organizations (name) values
    ('Tyrell Corporation') returning id into lastid;

  insert into users (name, email) values
    ('Gnusto', 'gnusto@tyrell.test'),
    ('Frotz', 'frotz@tyrell.test'),
    ('Yomin', 'yomin@tyrell.test'),
    ('Bozbar', 'bozbar@tyrell.test'),
    ('Rezrov', 'rezrov@tyrell.test');

  insert into topics (organization_id, description) values
    (lastid, 'Science'),
    (lastid, 'Biology'),
    (lastid, 'Chemistry'),
    (lastid, 'Physics');
end $$;
