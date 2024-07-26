use crate::functions::recipe::{
    create_recipe_query, delete_recipe_query, fuzzy_query, update_recipe_query,
};
use crate::functions::user::{
    create_user_record, delete_user_record, list_users_query, query_user_info, update_user_record,
};
use crate::structs::{
    FullRecipe, Ingredient, NewRecipe, NewUser, Recipe, Step, UrlRecipeQuery, UrlUserQuery,
};
use diesel::result::Error;
use diesel::Connection;
use std::env;

use crate::{db_pool::DieselError, structs::User};

use crate::db_pool::connect_to_db;
use crate::structs::UserRole;

const DEFAULT_DATABASE_URL: &'static str = "postgresql://dev:dev@localhost:5432/recipe_app";
fn get_db_url() -> String {
    env::var("DATABASE_URL").unwrap_or(String::from(DEFAULT_DATABASE_URL))
}

#[test]
fn test_create_user_record() {
    let pool = connect_to_db(get_db_url()).unwrap();
    let mut conn = pool.get().unwrap();
    let user = NewUser {
        user_name: "test_user".to_string(),
        user_role: UserRole::Admin,
        user_pwd: "password".to_string(),
    };
    conn.test_transaction::<_, Error, _>(|conn| -> Result<(), DieselError> {
        let old_len = list_users_query(conn)?.len();
        let u = create_user_record(conn, &user)?;
        println!("{:?}", u);
        let new_len = list_users_query(conn)?.len();

        assert!(new_len == old_len + 1);
        Ok(())
    });
}

#[test]
fn test_delete_user_record() {
    let pool = connect_to_db(get_db_url()).unwrap();
    let mut conn = pool.get().unwrap();
    let user = NewUser {
        user_name: "test_user".to_string(),
        user_role: UserRole::User,
        user_pwd: "password".to_string(),
    };
    conn.test_transaction::<_, Error, _>(|conn| -> Result<(), DieselError> {
        create_user_record(conn, &user)?;
        delete_user_record(
            conn,
            &UrlUserQuery {
                name: Some(user.user_name),
                ..Default::default()
            },
        )?;

        Ok(())
    })
}

#[test]
fn test_query_user_info() {
    let pool = connect_to_db(get_db_url()).unwrap();
    let mut conn = pool.get().unwrap();
    let user = NewUser {
        user_name: "test_user".to_string(),
        user_role: UserRole::User,
        user_pwd: "password".to_string(),
    };
    conn.test_transaction::<_, Error, _>(|conn| -> Result<(), DieselError> {
        create_user_record(conn, &user)?;

        let user = query_user_info(
            conn,
            &UrlUserQuery {
                name: Some("test_user".to_string()),
                ..Default::default()
            },
        )?;
        assert_eq!(user.user_name, "test_user");
        Ok(())
    });
}

#[test]
fn test_update_user_record() {
    let pool = connect_to_db(get_db_url()).unwrap();
    let mut conn = pool.get().unwrap();
    let old_user = NewUser {
        user_name: "test_user".to_string(),
        user_role: UserRole::Admin,
        user_pwd: "password".to_string(),
    };
    conn.test_transaction::<_, Error, _>(|conn| -> Result<(), DieselError> {
        let u = create_user_record(conn, &old_user)?;
        let new_info: User = User {
            id: u.id,
            user_name: "test_user2".to_string(),
            user_role: UserRole::User,
            user_pwd: "password".to_string(),
        };
        update_user_record(conn, &new_info)?;
        let query = UrlUserQuery {
            id: Some(u.id),
            ..Default::default()
        };
        let updated_user = query_user_info(conn, &query)?;
        assert_ne!(
            updated_user.user_name, old_user.user_name,
            "names are the same, user update OK"
        );
        Ok(())
    });
}

#[test]
fn test_list_users_query() {
    let pool = connect_to_db(get_db_url()).expect("failed to get pool");
    let mut conn = pool.get().expect("failed to get connection from pool");
    let users = vec![
        NewUser {
            user_name: "test_user".to_string(),
            user_role: UserRole::Admin,
            user_pwd: "password".to_string(),
        },
        NewUser {
            user_name: "test_user2".to_string(),
            user_role: UserRole::User,
            user_pwd: "password".to_string(),
        },
    ];
    conn.test_transaction::<_, Error, _>(|conn| -> Result<(), DieselError> {
        let old_len = list_users_query(conn)?.len();
        for user in &users {
            create_user_record(conn, user)?;
        }
        let new_len = list_users_query(conn)?;

        assert!(new_len.len() > old_len);
        Ok(())
    });
}

#[test]
fn create_recipe() {
    let pool = connect_to_db(get_db_url()).expect("failed to get pool");
    let mut conn = pool.get().expect("failed to get connection from pool");
    let input_recipe = NewRecipe::default();
    conn.test_transaction(|conn| {
        create_recipe_query(conn, &input_recipe)?;
        Ok::<_, DieselError>(())
    })
}

#[test]
fn test_delete_recipe() {
    let pool = connect_to_db(get_db_url()).expect("failed to get pool");
    let mut conn = pool.get().expect("failed to get connection from pool");
    let first_recipe = NewRecipe {
        recipe_name: String::from("value"),
        recipe_observations: None,
        ..Default::default()
    };
    let second_recipe = NewRecipe {
        recipe_observations: None,
        ..Default::default()
    };
    conn.test_transaction::<_, DieselError, _>(|conn| {
        create_recipe_query(conn, &first_recipe)?;
        let r2 = create_recipe_query(conn, &second_recipe)?;
        let old_len = fuzzy_query(conn, &String::from(""))?.len();
        println!("recipe table length after create_recipe {old_len}");
        delete_recipe_query(
            conn,
            &UrlRecipeQuery {
                id: Some(r2.id),
                name: None,
            },
        )?;
        assert!(
            fuzzy_query(conn, &String::from(""))?.len() == old_len - 1,
            "recipe 1 wasn't deleted!"
        );
        delete_recipe_query(
            conn,
            &UrlRecipeQuery {
                id: None,
                name: Some(first_recipe.recipe_name),
            },
        )?;
        assert!(
            fuzzy_query(conn, &String::from(""))?.len() == old_len - 2,
            "recipe 2 wasn't deleted!"
        );

        Ok(())
    })
}

#[test]
fn test_fuzzy_query() {
    let pool = connect_to_db(get_db_url()).expect("failed to get pool");
    let mut conn = pool.get().expect("failed to get connection from pool");

    conn.test_transaction::<_, DieselError, _>(move |conn| {
        let old_len = fuzzy_query(conn, &String::from(""))?.len();
        for i in 1..11 {
            create_recipe_query(
                conn,
                &NewRecipe {
                    user_id: 0,
                    recipe_name: format!("recipe{i}"),
                    recipe_observations: None,
                },
            )?;
        }
        let new_len = fuzzy_query(conn, &String::from(""))?.len();
        assert_eq!(new_len, old_len + 10);
        Ok(())
    })
}

#[test]
fn test_update_recipe() {
    let pool = connect_to_db(get_db_url()).expect("failed to get pool");
    let mut conn = pool.get().expect("failed to get connection from pool");
    let old_recipe = NewRecipe {
        recipe_name: "pao".to_string(),
        ..Default::default()
    };
    conn.test_transaction::<_, DieselError, _>(move |conn| {
        let created = create_recipe_query(conn, &old_recipe)?;

        let new_recipe = Recipe {
            id: created.id,
            recipe_name: String::from("tijolo"),
            ..Default::default()
        };
        let updated = update_recipe_query(conn, &new_recipe)?;
        assert_eq!(created.id, updated.id);
        assert_ne!(created.recipe_name, updated.recipe_name);
        Ok(())
    })
}

#[test]
fn test_fullrecipe_helpers() {
    let ingredients = vec![
        Ingredient {
            id: 10,
            ..Default::default()
        },
        Ingredient {
            id: 11,
            ..Default::default()
        },
        Ingredient {
            id: 12,
            ..Default::default()
        },
    ];
    let original_ingredients_len = ingredients.len();
    let steps = vec![
        Step {
            id: 10,
            ..Default::default()
        },
        Step {
            id: 11,
            ..Default::default()
        },
        Step {
            id: 13,
            ..Default::default()
        },
    ];
    let original_steps_len = steps.len();
    let mut full_recipe = FullRecipe {
        ingredients,
        steps,
        ..Default::default()
    };

    let new_i_len = full_recipe
        .remove_ingredient(10)
        .map_err(|err| panic!("{}", err))
        .unwrap();
    let new_s_len = full_recipe
        .remove_step(10)
        .map_err(|err| panic!("{}", err))
        .unwrap();
    assert!(new_s_len.len() == original_steps_len - 1);
    assert!(new_i_len.len() == original_ingredients_len - 1);

    let old_ingredient = full_recipe
        .get_ingredient(11)
        .map_err(|err| panic!("{}", err))
        .unwrap();
    let i = full_recipe
        .replace_ingredient(Ingredient {
            id: 11,
            ingredient_name: "new_name".to_string(),
            ..Default::default()
        })
        .map_err(|err| panic!("error: {err}"))
        .unwrap();
    full_recipe.set_ingredients(i);
    let updated_ingredient = full_recipe
        .get_ingredient(11)
        .map_err(|err| panic!("error! {err}"))
        .unwrap();
    assert_ne!(old_ingredient, updated_ingredient)
}
