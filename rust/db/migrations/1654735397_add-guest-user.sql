insert into users
  (id, name, primary_email, login, system, registered_at)
  values
  ('11a13e26-ee64-4c31-8af1-d1e953899ee0', 'Guest', 'noreply@nowehere', 'guest', 't', now())
  on conflict on constraint users_pkey do nothing;

insert into organization_members
  (user_id, organization_id)
  values
  ('11a13e26-ee64-4c31-8af1-d1e953899ee0', '45dc89a6-e6f0-11e8-8bc1-6f4d565e3ddb')
  on conflict on constraint organization_members_pkey do nothing;
