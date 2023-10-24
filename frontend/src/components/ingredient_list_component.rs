use db::structs::Ingredient;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IngredientsListProps {
    pub ingredients: Vec<Ingredient>,
}
#[function_component(IngredientList)]
pub fn ingredients_list(IngredientsListProps { ingredients }: &IngredientsListProps) -> Html {
    let l: Vec<Html> = ingredients
        .iter()
        .map(|ingredient| {
            html! {
                <li class="ingredient">
                    <div class="ingredient-name">{ingredient.ingredient_name.clone()}</div>
                    <div class="description">{format!("{} - {}",
                    ingredient.ingredient_quantity,ingredient.quantity_unit)}</div>
                </li>
            }
        })
        .collect();
    html! {
        <>
        <h2 class="ingredients">{"Ingredients"}</h2>
        <ul class="list">
            {l}
        </ul>
        </>
    }
}
