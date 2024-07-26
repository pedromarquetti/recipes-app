use db::structs::UserRole;
use jwt::UserClaims;
use routes::validate_permission;

use crate::{jwt, routes};

#[test]
fn test_validate_permission() {
    let normal_user = UserClaims {
        user_id: 1,
        role: UserRole::User,
        ..Default::default()
    };
    let admin_user = UserClaims {
        user_id: 2,
        role: UserRole::Admin,
        ..Default::default()
    };
    // checkig valid permissions
    assert_eq!(
        validate_permission(1, Some(admin_user)),
        true,
        " admin trying to do something"
    );
    assert_eq!(
        validate_permission(1, Some(normal_user.clone())),
        true,
        "owner trying to do something"
    );

    assert_eq!(
        validate_permission(2, Some(normal_user.clone())),
        false,
        "normal user trying to modify something"
    );

    assert_eq!(
        validate_permission(-1, Some(normal_user.clone())),
        false,
        "Invalid ID!"
    );

    assert_eq!(validate_permission(1, None), false, "No claims!");
}
