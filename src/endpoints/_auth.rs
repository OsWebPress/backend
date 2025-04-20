use actix_web::guard;
use crate::jwt;
use crate::database;

/* The user must have a Bearer token with a valid role. */
pub fn role_guard(ctx: &guard::GuardContext) -> bool {
    if let Some(claims) = ctx.req_data().get::<jwt::Claims>() {
        println!("Claims: User Role: {}", claims.role);
    } else {
        println!("No claims found");
        return false;
    }
    return true;
}

/* The user must have a Bearer token with the Admin role. */
pub fn admin_guard(ctx: &guard::GuardContext) -> bool {
    if let Some(claims) = ctx.req_data().get::<jwt::Claims>() {
		return claims.role == database::Role::Admin;
	} else {
        return false;
    }
}
