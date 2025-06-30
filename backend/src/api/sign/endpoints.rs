use axum::{Json, extract::State, response::IntoResponse};
use http::StatusCode;
use tracing::{error, info};

use crate::{
    api::{
        sign::{
            models::{SignPayload, UserSignResponse},
            services::{check_cookie_exists, get_cookies, sign_fn},
        },
        users::get_users_by_ulids,
    },
    misc::GlobalState,
};

#[utoipa::path(
    post,
    path = "/api/sign",
    description = "Sign cookies for the provided ULIDs and URL",
    request_body = SignPayload,
    responses(
        (status = 200, description = "Cookies signed successfully", body = Vec<UserSignResponse>),
        (status = 400, description = "No users found for the provided ULIDs"),
        (status = 401, description = "Unauthorized - Invalid or missing JWT token"),
        (status = 404, description = "No cookies found for today"),
    ),
    tag = "Sign"
)]
pub async fn sign(
    State(state): State<GlobalState>,
    Json(payload): Json<SignPayload>,
) -> impl IntoResponse {
    let cookies = match get_cookies(&state) {
        Ok(Some(cookies)) => cookies,
        Ok(None) => {
            return (StatusCode::NOT_FOUND, "No cookies found for today").into_response();
        }
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error fetching cookies: {}", err),
            )
                .into_response();
        }
    };

    let users = match get_users_by_ulids(&state, &payload.ulids) {
        Ok(users) => users,
        Err(err) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Error fetching users: {}", err),
            )
                .into_response();
        }
    };

    if users.len() != payload.ulids.len() {
        error!(
            "Mismatch in number of users found: expected {}, found {}",
            payload.ulids.len(),
            users.len()
        );
        return (
            StatusCode::BAD_REQUEST,
            "No users found for the provided ULIDs",
        )
            .into_response();
    }

    let signing_result = sign_fn(cookies, users, &payload.url).await;
    println!("Signing result: {:?}", signing_result);

    match signing_result {
        Ok(res) => (StatusCode::OK, Json(res)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/api/sign/status",
    description = "Check the status of cookies for today",
    responses(
        (status = 200, description = "Cookies exist for today"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "No cookies found for today"),
    ),
    tag = "Sign"
)]
pub async fn status(State(state): State<GlobalState>) -> impl IntoResponse {
    info!("Checking status...");
    let today = chrono::Local::now().date_naive();
    match check_cookie_exists(&state, today) {
        Ok(true) => (StatusCode::OK, "Cookies exist for today".to_string()).into_response(),
        Ok(false) => (
            StatusCode::NOT_FOUND,
            "No cookies found for today".to_string(),
        )
            .into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error checking cookies: {}", err),
        )
            .into_response(),
    }
}
