// @generated automatically by Diesel CLI.

diesel::table! {
    recipe (id) {
        id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        recipe_name -> Varchar,
        recipe_ingredients -> Array<Text>,
        recipe_observations -> Nullable<Array<Text>>,
    }
}

diesel::table! {
    recipe_step (id) {
        id -> Nullable<Int4>,
        recipe_id -> Int4,
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
    }
}

diesel::joinable!(recipe -> recipe_users (user_id));
diesel::joinable!(recipe_step -> recipe (recipe_id));

diesel::allow_tables_to_appear_in_same_query!(recipe, recipe_step, recipe_users,);
