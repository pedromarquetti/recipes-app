CREATE TABLE IF NOT EXISTS recipe_users (
    id SERIAL PRIMARY KEY,
    user_name VARCHAR UNIQUE NOT NULL,
    user_pwd TEXT NOT NULL,
    user_role TEXT NOT NULL
);
/* creating default admin user */
INSERT INTO recipe_users (id, user_name, user_pwd, user_role)
VALUES (
    0,
    'admin',
    '$2a$04$Plkgo7MNUzowxl8/r0tFMeQE2E8DDej6oKkuqXMr5EIUcZKTSGGIS',
    'admin'
) ON CONFLICT DO NOTHING;