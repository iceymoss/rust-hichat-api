use axum::Router;

pub struct AppRouter {
    pub router: Router,
}

impl AppRouter {
    pub fn new() -> Self {
        AppRouter {
            router: Router::new()
        }
    }
}