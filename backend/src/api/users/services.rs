use axum::{
    Router,
    routing::{get, post},
};
use chrono::NaiveDateTime;
use ulid::Ulid;

use super::models::User;
use crate::{api::auth::RegisterPayload, misc::GlobalState};

pub fn get_routes(state: GlobalState) -> Router {
    Router::new()
        .route("/me", get(super::endpoints::get_me))
        .route("/me/update-jwt", post(super::endpoints::update_jwt))
        .with_state(state)
}

pub fn create_user(state: &GlobalState, user: User) -> Result<(), diesel::result::Error> {
    use crate::schema::users;
    use diesel::prelude::*;

    let mut conn = match state.get_db_conn() {
        Ok(conn) => conn,
        Err(_) => return Err(diesel::result::Error::NotFound),
    };

    diesel::insert_into(users::table)
        .values(&user)
        .execute(&mut conn)?;

    Ok(())
}

pub fn user_exists(
    state: &GlobalState,
    user: &RegisterPayload,
) -> Result<(), diesel::result::Error> {
    use crate::schema::users::dsl::*;
    use diesel::prelude::*;

    let mut conn = match state.get_db_conn() {
        Ok(conn) => conn,
        Err(_) => return Err(diesel::result::Error::NotFound),
    };

    let user_exists = users
        .filter(username.eq(&user.username))
        .select(User::as_select())
        .first::<User>(&mut conn)
        .optional()?;

    if user_exists.is_some() {
        return Err(diesel::result::Error::DatabaseError(
            diesel::result::DatabaseErrorKind::UniqueViolation,
            Box::new("User already exists".to_string()),
        ));
    }

    Ok(())
}

pub fn get_user_by_username(
    state: &GlobalState,
    query_username: &str,
) -> Result<Option<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    use diesel::prelude::*;

    let mut conn = match state.get_db_conn() {
        Ok(conn) => conn,
        Err(_) => return Err(diesel::result::Error::NotFound),
    };

    users
        .filter(username.eq(query_username))
        .select(User::as_select())
        .first(&mut conn)
        .optional()
}

pub fn update_user_jwt(
    state: &GlobalState,
    user_id: Ulid,
    new_jwt: &str,
    new_jwt_exp: NaiveDateTime,
) -> Result<(), diesel::result::Error> {
    use crate::schema::users::dsl::*;
    use diesel::prelude::*;

    let mut conn = match state.get_db_conn() {
        Ok(conn) => conn,
        Err(_) => return Err(diesel::result::Error::NotFound),
    };

    diesel::update(users.filter(id.eq(user_id.to_string())))
        .set((
            jwt_intra_epitech.eq(new_jwt),
            jwt_expires_at.eq(new_jwt_exp),
        ))
        .execute(&mut conn)?;

    Ok(())
}

pub fn get_users_by_ulids(
    state: &GlobalState,
    user_ids: &Vec<Ulid>,
) -> Result<Vec<User>, diesel::result::Error> {
    use crate::schema::users::dsl::*;
    use diesel::prelude::*;

    let mut conn = match state.get_db_conn() {
        Ok(conn) => conn,
        Err(_) => return Err(diesel::result::Error::NotFound),
    };

    let user_id_strings: Vec<String> = user_ids
        .iter()
        .map(|other_id| other_id.to_string())
        .collect();

    users
        .filter(id.eq_any(user_id_strings))
        .select(User::as_select())
        .load(&mut conn)
}
