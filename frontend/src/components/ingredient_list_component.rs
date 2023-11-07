use db::structs::Ingredient;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IngredientsListProps {
    pub ingredients: Vec<Ingredient>,
}
#[function_component(IngredientList)]
pub fn ingredients_list(IngredientsListProps { ingredients }: &IngredientsListProps) -> Html {
    let ingredient_list: Vec<Html> = ingredients
        .iter()
        .map(|ingredient| {
            html! {
                <IngredientItem ingredient={ingredient.clone()} />
            }
        })
        .collect();
    html! {
        <>
        <h2 class="ingredients">{"Ingredients"}</h2>
        <ul class="list">
            {ingredient_list}
        </ul>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct IngredientItemProp {
    pub ingredient: Ingredient,
}

#[function_component(IngredientItem)]
/// Component used to represent a Ingredient item
pub fn ingredient(ingredient: &IngredientItemProp) -> Html {
    html! {
        <li class="ingredient">
            {ingredient.ingredient.ingredient_name.clone()}
            {format!(" {} {}",
            ingredient.ingredient.ingredient_quantity,ingredient.ingredient.quantity_unit)}
        </li>
    }
}
