use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IngredientsListProps {
    pub ingredients: Vec<String>,
}
#[function_component(IngredientList)]
pub fn ingredients_list(IngredientsListProps { ingredients }: &IngredientsListProps) -> Html {
    let l: Vec<Html> = ingredients
        .iter()
        .map(|ingredient| {
            html! {
                <li class="ingredient">{ingredient}</li>
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
