CREATE TABLE IF NOT EXISTS recipe_users (
    id SERIAL PRIMARY KEY,
    user_name VARCHAR UNIQUE NOT NULL,
    user_pwd TEXT NOT NULL,
    user_role TEXT NOT NULL
);