CREATE TABLE IF NOT EXISTS recipe_step (
    id SERIAL PRIMARY KEY,
    recipe_id INT NOT NULL,
    step_name CHAR(50) NOT NULL,
    step_instruction TEXT NOT NULL,
    step_duration_min INT NOT NULL,
    CONSTRAINT fk_recipe_id FOREIGN KEY (recipe_id) REFERENCES recipe(id) on delete cascade on update cascade
);