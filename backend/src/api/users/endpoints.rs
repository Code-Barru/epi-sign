use axum::{extract::State, response::IntoResponse, Json};
use chrono::{DateTime};
use http::StatusCode;
use base64::{Engine as _, engine::general_purpose};
use serde_json::Value;

use crate::{api::{auth::JwtClaims, users::{get_user_by_username, models::JwtPayload, services::update_user_jwt, User}}, misc::GlobalState};

#[utoipa::path(
    get,
    path = "/api/users/me",
    description = "Get the current user's information",
    responses(
        (status = 200, description = "User information retrieved successfully", body = User),
        (status = 404, description = "User not found"),
        (status= 401, description = "Unauthorized - Invalid or missing JWT token")
    ),
    tag = "Users"
)]
pub async fn get_me(State(state): State<GlobalState>, jwt: JwtClaims) -> impl IntoResponse {
    match get_user_by_username(&state, &jwt.name) {
        Ok(user) => Json(user).into_response(),
        Err(_) => return (StatusCode::NOT_FOUND).into_response()
    }
}

#[utoipa::path(
    post,
    path = "/api/users/me/update-jwt",
    description = "Update the JWT for the current user",
    request_body = JwtPayload,
    responses(
        (status = 200, description = "JWT updated successfully"),
        (status = 400, description = "Invalid JWT format or payload"),
        (status = 401, description = "Unauthorized"),
    ),
    tag = "Users"
)]
pub async fn update_jwt(State(state): State<GlobalState>, jwt_user: JwtClaims, Json(jwt_payload): Json<JwtPayload>) -> impl IntoResponse {
    let parts: Vec<&str> = jwt_payload.jwt.split('.').collect();
    
    if parts.len() != 3 {
        return (StatusCode::BAD_REQUEST, "Invalid JWT format").into_response();
    }

    let decoded = match general_purpose::URL_SAFE_NO_PAD.decode(parts[1]) {
        Ok(decoded) => decoded,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid JWT payload").into_response(),
    };

    let exp = match serde_json::from_slice::<Value>(&decoded) {
        Ok(payload) => payload.get("exp").cloned(),
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid JWT payload format").into_response(),
    };

    let exp = match exp {
        Some(exp) => match exp.as_i64() {
            Some(exp) => exp,
            None => return (StatusCode::BAD_REQUEST, "Expiration time is not an integer").into_response(),
        },
        None => return (StatusCode::BAD_REQUEST, "Expiration time not found in JWT payload").into_response(),
    };

    if exp <= 0 {
        return (StatusCode::BAD_REQUEST, "Invalid expiration time in JWT payload").into_response();
    }

    let exp_datetime = match DateTime::from_timestamp(exp, 0) {
        Some(datetime) => datetime,
        None => return (StatusCode::BAD_REQUEST, "Invalid timestamp").into_response(),
    };

    let exp_naive = exp_datetime.naive_utc();    

    match update_user_jwt(&state, jwt_user.sub, &jwt_payload.jwt, exp_naive) {
        Ok(_) => (StatusCode::OK).into_response(),
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
        }
    }
}