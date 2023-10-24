CREATE TABLE IF NOT EXISTS recipe_ingredient (
    id SERIAL PRIMARY KEY,
    recipe_id INT NOT NULL,
    ingredient_name  VARCHAR(100) NOT NULL,
    ingredient_quantity  INT NOT NULL,
    quantity_unit  VARCHAR(10) NOT NULL,
    CONSTRAINT fk_recipe_ingredient FOREIGN KEY (recipe_id) REFERENCES recipe(id) ON DELETE CASCADE ON UPDATE CASCADE
)