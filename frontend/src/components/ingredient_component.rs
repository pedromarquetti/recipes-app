use db::structs::Ingredient;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IngredientItemProp {
    pub ingredient: Ingredient,
}

#[function_component(IngredientItem)]
/// Component used to represent a Ingredient item
pub fn ingredient_component(ingredient: &IngredientItemProp) -> Html {
    html! {
        <div class="ingredient">
            {ingredient.ingredient.ingredient_name.clone()}
            {format!(" {} {}",
            ingredient.ingredient.ingredient_quantity,ingredient.ingredient.quantity_unit)}
        </div>
    }
}
