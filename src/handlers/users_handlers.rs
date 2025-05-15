use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::Json;
use sea_orm::DbErr;
use crate::{
    models::users::{CreateUserRequest, UserResponse},
    models::users::{ListRequest, ListResponse, UpdateUserRequest},
    services::user_service::UserService,
};
// 不使用模式匹配解构 State
// 不使用模式匹配解构 Json
pub async fn create_user(payload: axum::Json<CreateUserRequest>) -> Result<axum::Json<UserResponse>, (axum::http::StatusCode, String)> {
    // 显式获取服务需要的参数
    let request_data = payload.0;  // 从 Json 解包请求体数据

    // 显式处理异步调用
    let service_result = UserService::create_user(request_data).await;

    // 使用 match 代替 map/map_err
    match service_result {
        Ok(user_response) => {
            let json_response = axum::Json(user_response);
            Ok(json_response)
        }
        Err(error) => {
            let error_message = error.to_string();
            Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, error_message))
        }
    }
}

// Query 提取器 
pub async fn list_users(Query(params): Query<ListRequest>) -> Result<Json<ListResponse>, (StatusCode, String)> {
    if params.page < 1 || params.page_size < 1 {
        return Err((
            StatusCode::BAD_REQUEST,
            "页码和分页大小必须大于0".to_string()
        ));
    }

    if params.page_size > 100 {
        return Err((
            StatusCode::BAD_REQUEST,
            "每页最多显示100条数据".to_string()
        ));
    }

    // 调用业务逻辑
    match UserService::list_users(params).await {
        Ok(res) => Ok(Json(res)),
        Err(err) => {
            // 将数据库错误映射到HTTP状态码
            let (status_code, error_msg) = match err {
                DbErr::RecordNotFound(msg) => (StatusCode::NOT_FOUND, msg),
                DbErr::Conn(_) => (
                    StatusCode::SERVICE_UNAVAILABLE,
                    "数据库连接失败".to_string()
                ),
                DbErr::Query(_) => (
                    StatusCode::BAD_REQUEST,
                    "无效的查询请求".to_string()
                ),
                DbErr::Custom(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "服务器内部错误".to_string()
                )
            };

            Err((status_code, error_msg))
        }
    }
}

pub async fn delete_user(Path(user_id): Path<uuid::Uuid>) -> Result<Json<()>, (StatusCode, String)> {
    if user_id.is_nil() {
        return Err((StatusCode::BAD_REQUEST, "id 为空".to_string()));
    }
    
    match UserService::delete_user(user_id).await { 
        Ok(res) => Ok(Json(res)),
        Err(err) => {
            // 将数据库错误映射到HTTP状态码
            let (status_code, error_msg) = match err {
                DbErr::RecordNotFound(msg) => (StatusCode::NOT_FOUND, msg),
                DbErr::Conn(_) => (
                    StatusCode::SERVICE_UNAVAILABLE,
                    "数据库连接失败".to_string()
                ),
                DbErr::Query(_) => (
                    StatusCode::BAD_REQUEST,
                    "无效的查询请求".to_string()
                ),
                DbErr::Custom(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "服务器内部错误".to_string()
                )
            };

            Err((status_code, error_msg))
        }
    }
}

pub async fn update_user(Path(user_id): Path<uuid::Uuid>, Json(payload): Json<UpdateUserRequest>) -> Result<Json<()>, (StatusCode, String)> {
    if user_id.is_nil() {
        return Err((StatusCode::BAD_REQUEST, "id 为空".to_string()));
    }

    if payload.username.is_none() && payload.email.is_none() && payload.state.is_none() {
        return Err((
            StatusCode::BAD_REQUEST,
            "至少需要提供一个更新字段".to_string()
        ));
    }
    
    match UserService::update_user(user_id, payload).await {
        Ok(res) => Ok(Json(res)),
        Err(e) => {
            let (status, msg) = match e {
                DbErr::RecordNotFound(msg) => (StatusCode::NOT_FOUND, msg),
                DbErr::Custom(msg) if msg.contains("无效") => (StatusCode::BAD_REQUEST, msg),
                DbErr::Custom(msg) => (StatusCode::BAD_REQUEST, msg),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "用户更新失败".to_string()
                ),
            };
            Err((status, msg))
        }
    }
}

pub async fn get_user(Path(user_id): Path<uuid::Uuid>) -> Result<Json<UserResponse>, (StatusCode, String)> {
    if user_id.is_nil() {
        return Err((StatusCode::BAD_REQUEST, "id 为空".to_string()));
    }
    
    match UserService::get_user(user_id).await {
        Ok(user) => Ok(Json(user)),
        Err(err) => {
            // 将数据库错误映射到HTTP状态码
            let (status_code, error_msg) = match err {
                DbErr::RecordNotFound(msg) => (StatusCode::NOT_FOUND, msg),
                DbErr::Conn(_) => (
                    StatusCode::SERVICE_UNAVAILABLE,
                    "数据库连接失败".to_string()
                ),
                DbErr::Query(_) => (
                    StatusCode::BAD_REQUEST,
                    "无效的查询请求".to_string()
                ),
                DbErr::Custom(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "服务器内部错误".to_string()
                )
            };

            Err((status_code, error_msg))
        }
    }
    
    
}

