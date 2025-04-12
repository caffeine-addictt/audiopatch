use axum::{Router, routing::get};
pub fn routes() -> Router {
    Router::new()
        .fallback(notfound::handler)
}

pub mod notfound;
