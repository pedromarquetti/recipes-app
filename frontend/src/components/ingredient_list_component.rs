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
        <h1 class="ingredients">{"ingredients"}</h1>
        <ul class="list">
            {l}
        </ul>
        </>
    }
}
