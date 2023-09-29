// cfg changes how the code will be compiled
#[cfg(not(target_arch = "wasm32"))]
pub mod db_pool;
#[cfg(not(target_arch = "wasm32"))]
pub mod functions;
pub mod schema;

pub mod structs;

#[cfg(test)]
mod tests {

    use serde_json::{json, Value};

    use crate::structs::FullRecipe;

    #[test]
    /// parse full recipe from Value
    fn full_recipe_parse() {
        let val: Value = json!(
        {
            "recipe":{
                "id":2,"user_id":null,"recipe_name":"Batata Frita","recipe_ingredients":["batata","oleo"],"recipe_observations":null
            },
            "steps":
            [{"id":8,"recipe_id":2,"step_name":"esquentar oleo","step_instruction":"esquentar oleo","step_duration_min":6},{"id":9,"recipe_id":2,"step_name":"esquentar oleo                                    ","step_instruction":"esquentar oleo","step_duration_min":6}]
        });
        let full_recipe: FullRecipe = serde_json::from_value(val.clone()).expect("expected recipe");
        assert!(full_recipe.recipe.id.is_some())
    }
}
