pub mod edit_ingredients;
pub mod edit_mode;
pub mod edit_steps;
pub mod ingredient_component;
pub mod input_component;
pub mod navbar_component;
pub mod new_ingredient;
pub mod new_step;
pub mod recipe_card_component;
pub mod recipe_component;
pub mod recipe_title;
pub mod steps_component;

use db::structs::RecipeTrait;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
/// `old_part` is a placeholder for the part O that will be modified
///
/// `handle_edit` and `handle_new` are used for different components
///
/// `T` Represents what type will be used in the fields
pub struct RecipePartProps<T>
where
    T: PartialEq + RecipeTrait + Default,
{
    #[prop_or_default]
    pub callback: Callback<(RecipeMode, T)>,
    #[prop_or_default]
    /// old Recipe/Ingredient/Step...
    pub old_part: T,
    #[prop_or_default]
    pub recipe_id: i32,
}
#[derive(PartialEq, Clone, Debug)]
pub enum RecipeMode {
    View,
    Edit,
    New,
    Delete,
}
