use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use axum_cookie::CookieManager;

pub const DEV_TOKEN: &str = "11dc7bac0042b71d690a29b2c2787ccb89f9014471902b5667daff7c9b407c9c";

pub fn get_nest() -> Router {
    Router::new().route("/dev", post(dev_auth))
}

async fn dev_auth(cookie: CookieManager) -> Result<impl IntoResponse, StatusCode> {
    if !cfg!(debug_assertions) {
        return Err(StatusCode::FORBIDDEN);
    }

    cookie.add(("token", DEV_TOKEN).into());

    Ok(StatusCode::OK)
}
