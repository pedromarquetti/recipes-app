// @generated automatically by Diesel CLI.

diesel::table! {
    recipe (id) {
        id -> Int4,
        user_id -> Nullable<Int4>,
        #[max_length = 100]
        recipe_name -> Varchar,
        recipe_observations -> Nullable<Array<Nullable<Text>>>,
    }
}

diesel::table! {
    recipe_ingredient (id) {
        id -> Int4,
        recipe_id -> Int4,
        #[max_length = 100]
        ingredient_name -> Varchar,
        ingredient_quantity -> Int4,
        #[max_length = 10]
        quantity_unit -> Varchar,
    }
}

diesel::table! {
    recipe_step (id) {
        id -> Int4,
        recipe_id -> Int4,
        #[max_length = 50]
        step_name -> Bpchar,
        step_instruction -> Text,
        step_duration_min -> Int4,
    }
}

diesel::table! {
    recipe_users (id) {
        id -> Int4,
        user_name -> Varchar,
        user_pwd -> Text,
        user_role -> Text,
    }
}

diesel::joinable!(recipe -> recipe_users (user_id));
diesel::joinable!(recipe_ingredient -> recipe (recipe_id));
diesel::joinable!(recipe_step -> recipe (recipe_id));

diesel::allow_tables_to_appear_in_same_query!(recipe, recipe_ingredient, recipe_step, recipe_users,);
