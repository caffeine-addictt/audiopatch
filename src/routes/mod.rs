use axum::{Router, routing::get};
pub fn routes() -> Router {
    Router::new()
        .route("/", get(index::handler))
        .fallback(notfound::handler)
}

pub mod index;
pub mod notfound;
