use db::structs::{FullRecipe, Ingredient, Step};
use yew::prelude::*;

use crate::components::{
    ingredient_component::IngredientItem, recipe_title::RecipeTitle, steps_component::StepItem,
};

#[derive(Properties, PartialEq)]
pub struct RecipeProps {
    pub full_recipe: FullRecipe,
}
#[function_component(RecipeComponent)]
/// Base Recipe component
pub fn recipe_component(RecipeProps { full_recipe }: &RecipeProps) -> Html {
    let recipe = full_recipe.recipe.clone();
    let ingredients = full_recipe.ingredients.clone();
    let steps = full_recipe.steps.clone();
    html! {
        <div class="recipe">
            <RecipeTitle owner={full_recipe.recipe_owner_name.clone()} title={recipe.recipe_name}/>
            <IngredientList ingredients={
                    ingredients
                }/>
            <StepList step_list={steps}/>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct StepListProps {
    pub step_list: Vec<Step>,
}
#[function_component(StepList)]
pub fn step_list(steps: &StepListProps) -> Html {
    let l: Vec<Html> = steps
        .step_list
        .iter()
        .map(|step: &Step| {
            // note: calling unwrap on step.id because step will always receive an id
            html! {
                <li id={format!("step-{}",step.id.unwrap_or(steps.step_list.len().try_into().expect("invalid len()")))} class="step">
                    <StepItem step={step.clone()}/>
                </li>
            }
        })
        .collect();

    html! {
    <>
        <h2 >{"Steps"}</h2>
        <ol class="list">
            {l}
        </ol>
    </>
    }
}

#[derive(Properties, PartialEq)]
pub struct IngredientsListProps {
    pub ingredients: Vec<Ingredient>,
}
#[function_component(IngredientList)]
/// Represents a list of ingredients
///
/// Uses a vec![Ingredient] as a prop.
pub fn ingredients_list(IngredientsListProps { ingredients }: &IngredientsListProps) -> Html {
    let ingredient_list: Vec<Html> = ingredients
        .iter()
        .map(|ingredient| {
            html! {
                <li id={format!{"ingredient-{}",ingredient.id.unwrap_or(ingredients.len().try_into().expect("invelid len()"))}}>
                    <IngredientItem ingredient={ingredient.clone()} />
                </li>
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
