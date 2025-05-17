use axum::{routing::post, Router};
use axum::routing::{delete, get, put};
use crate::handlers::users_handlers;

pub fn user_routes(router: Router) -> Router {
    router.nest("/api/users",
                Router::new()
                    // 创建用户（POST /api/users）
                    .route("/", post(users_handlers::create_user))
                    // 获取用户列表（GET /api/users?page=1&size=10）
                    .route("/", get(users_handlers::list_users))
                    // 获取单个用户（GET /api/users/{id}）
                    .route("/:id", get(users_handlers::get_user))
                    .route("/:id", delete(users_handlers::delete_user))
                    .route("/:id", put(users_handlers::update_user))
    )
}