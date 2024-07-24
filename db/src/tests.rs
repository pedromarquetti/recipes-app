use crate::functions::user::{
    create_user_record, delete_user_record, list_users_query, query_user_info, update_user_record,
};
use crate::structs::UrlUserQuery;
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
    let user = User {
        id: Some(1),
        user_name: "test_user".to_string(),
        user_role: UserRole::Admin,
        user_pwd: "password".to_string(),
    };
    conn.test_transaction::<_, Error, _>(|conn| -> Result<(), DieselError> {
        let old_len = list_users_query(conn)?.len();
        create_user_record(conn, &user)?;
        let new_len = list_users_query(conn)?.len();

        assert!(new_len > old_len);
        Ok(())
    });
}

#[test]
fn test_delete_user_record() {
    let pool = connect_to_db(get_db_url()).unwrap();
    let mut conn = pool.get().unwrap();
    let user = User {
        id: Some(1),
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
    let user = User {
        id: Some(1),
        user_name: "test_user".to_string(),
        user_role: UserRole::Admin,
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
    let old_user = User {
        id: Some(1),
        user_name: "test_user".to_string(),
        user_role: UserRole::Admin,
        user_pwd: "password".to_string(),
    };
    conn.test_transaction::<_, Error, _>(|conn| -> Result<(), DieselError> {
        create_user_record(conn, &old_user)?;
        let new_info: User = User {
            id: old_user.id,
            user_name: "test_user2".to_string(),
            user_role: UserRole::User,
            user_pwd: "password".to_string(),
        };
        update_user_record(conn, &new_info)?;
        let query = UrlUserQuery {
            id: old_user.id,
            ..Default::default()
        };
        let updated_user = query_user_info(conn, &query)?;
        assert_eq!(
            updated_user.user_name, new_info.user_name,
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
        User {
            id: Some(1),
            user_name: "test_user".to_string(),
            user_role: UserRole::Admin,
            user_pwd: "password".to_string(),
        },
        User {
            id: Some(2),
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
