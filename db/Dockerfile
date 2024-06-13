FROM postgres:latest


ADD db/migrations/1_users/up.sql /docker-entrypoint-initdb.d/1_users.sql
ADD db/migrations/2_recipe/up.sql /docker-entrypoint-initdb.d/2_recipe.sql
ADD db/migrations/3_recipe_steps/up.sql /docker-entrypoint-initdb.d/3_recipe_steps.sql
ADD db/migrations/4_recipe_ingredient/up.sql /docker-entrypoint-initdb.d/4_recipe_ingredient.sql
