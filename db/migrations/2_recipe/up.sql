CREATE TABLE IF NOT EXISTS recipe (
    id SERIAL PRIMARY KEY,
    user_id INT NULL,
    recipe_name VARCHAR(100) NOT NULL UNIQUE,
    recipe_ingredients  TEXT[] NOT NULL,
    recipe_observations TEXT[] ,
    CONSTRAINT fk_recipe_owner  FOREIGN KEY (user_id) REFERENCES recipe_users(id) on delete cascade on update cascade

);