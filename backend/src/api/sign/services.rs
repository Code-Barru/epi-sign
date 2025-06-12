use crate::{
    api::{
        sign::models::{Cookie, CookieItem, SignResponse, UserSignResponse},
        users::User,
    },
    misc::GlobalState,
};
use axum::routing::Router;
use http::{StatusCode, header::COOKIE};
use tracing::error;

pub fn get_routes(state: GlobalState) -> Router {
    Router::new()
        .route("/", axum::routing::post(super::endpoints::sign))
        .route("/status", axum::routing::get(super::endpoints::status))
        .with_state(state)
}

pub fn check_cookie_exists(
    state: &GlobalState,
    query_date: chrono::NaiveDate,
) -> Result<bool, String> {
    use crate::schema::cookies::dsl::*;
    use diesel::prelude::*;

    let mut conn = match state.get_db_conn() {
        Ok(conn) => conn,
        Err(_) => return Err("Failed to get database connection".into()),
    };

    let exists = diesel::select(diesel::dsl::exists(cookies.filter(date.eq(query_date))))
        .get_result::<bool>(&mut conn)
        .map_err(|_| "Database error when checking cookie existence".to_string())?;

    Ok(exists)
}

pub fn get_cookies(state: &GlobalState) -> Result<Option<Vec<CookieItem>>, String> {
    use crate::schema::cookies::dsl::*;
    use diesel::prelude::*;

    let current_date = chrono::Utc::now().date_naive();

    let mut conn = match state.get_db_conn() {
        Ok(conn) => conn,
        Err(_) => return Err("Failed to get database connection".into()),
    };

    let cookie_record = cookies
        .filter(date.eq(current_date))
        .select(Cookie::as_select())
        .first::<Cookie>(&mut conn)
        .optional();

    let cookie = match cookie_record {
        Ok(Some(cookie)) => cookie,
        Ok(None) => {
            return Ok(None);
        }
        Err(_) => {
            return Err("Database error when fetching cookies".into());
        }
    };

    match cookie.cookie_data.as_str() {
        Some(json_string) => match serde_json::from_str::<Vec<CookieItem>>(json_string) {
            Ok(cookie_items) => Ok(Some(cookie_items)),
            Err(_) => {
                return Err("Failed to parse cookie JSON string".into());
            }
        },
        None => Err("Cookie data is not a valid JSON string".into()),
    }
}

pub async fn sign_fn(
    cookies: Vec<CookieItem>,
    users: Vec<User>,
    url: &str,
) -> Result<Vec<UserSignResponse>, String> {
    let mut res = Vec::new();
    let (url, token) = get_parsed_url(url)?;
    let payload = serde_json::json!({
        "token": token,
        "rate": 0,
        "comment": ""
    });

    let client = match get_reqwest_client() {
        Ok(client) => client,
        Err(e) => {
            return Err(format!("Failed to create HTTP client: {}", e));
        }
    };
    for user in users {
        let jwt_cookie = match user.get_jwt_as_cookie() {
            Some(cookie) => cookie,
            None => {
                res.push(UserSignResponse {
                    ulid: user.id,
                    response: SignResponse::TokenNotFound,
                });
                continue;
            }
        };
        let mut user_cookies = cookies.clone();
        user_cookies.push(jwt_cookie);
        let cookie_str = user_cookies
            .iter()
            .map(|c| c.to_header_value())
            .collect::<Vec<_>>()
            .join("; ");

        let response = client
            .post(url.clone())
            .header(COOKIE, cookie_str)
            .json(&payload)
            .send()
            .await;

        let status = match response {
            Ok(resp) => resp.status(),
            Err(e) => {
                error!("HTTP request failed: {}", e);
                res.push(UserSignResponse {
                    ulid: user.id,
                    response: SignResponse::UnknownError,
                });
                continue;
            }
        };

        match status {
            StatusCode::OK => res.push(UserSignResponse {
                ulid: user.id,
                response: SignResponse::Success,
            }),
            StatusCode::UNAUTHORIZED => res.push(UserSignResponse {
                ulid: user.id,
                response: SignResponse::TokenExpired,
            }),
            StatusCode::FORBIDDEN => res.push(UserSignResponse {
                ulid: user.id,
                response: SignResponse::TokenExpired,
            }),
            StatusCode::INTERNAL_SERVER_ERROR => {
                res.push(UserSignResponse {
                    ulid: user.id,
                    response: SignResponse::AlreadySigned,
                });
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                res.push(UserSignResponse {
                    ulid: user.id,
                    response: SignResponse::ServiceUnavailable,
                });
            }
            _ => {
                error!("Unexpected status code: {}", status);

                res.push(UserSignResponse {
                    ulid: user.id,
                    response: SignResponse::UnknownError,
                });
            }
        }
    }

    Ok(res)
}

pub fn get_parsed_url(url: &str) -> Result<(reqwest::Url, String), String> {
    let url = reqwest::Url::parse(url).map_err(|e| format!("Failed to parse URL: {}", e))?;

    let token = url
        .query_pairs()
        .find(|(k, _)| k == "token")
        .map(|(_, v)| v.to_string())
        .ok_or_else(|| "Token not found in URL".to_string())?;

    let mut path_segments: Vec<_> = url
        .path_segments()
        .map(|c| c.collect::<Vec<_>>())
        .unwrap_or_default();

    if let Some(last) = path_segments.last() {
        if *last == "registered" {
            path_segments.pop();
        }
    }
    path_segments.push("token");

    let mut new_url = url.clone();
    new_url.set_path(&path_segments.join("/"));
    new_url.set_query(Some("format=json"));

    Ok((new_url, token))
}

pub fn get_reqwest_client() -> Result<reqwest::Client, String> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static("Mozilla/5.0 (compatible; MyApp/1.0)"),
    );
    headers.insert(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/json"),
    );

    reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))
}
