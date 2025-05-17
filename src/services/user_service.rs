use chrono::Utc;
use uuid::Uuid;
use rand::rngs::OsRng;
use sea_orm::{ActiveModelTrait, DbErr, ActiveValue::Set, EntityTrait, QueryFilter, ColumnTrait, PaginatorTrait, QuerySelect};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};

use crate::{config::database::get_db, entities::users::{ActiveModel, Model as UserModel},
            models::users::{CreateUserRequest, UserResponse},
            models::users::{ListRequest, ListResponse, UpdateUserRequest},
            entities::users::{self, Entity as Users},
};

pub struct UserService;

impl UserService {
    pub async fn create_user(req: CreateUserRequest) -> Result<UserResponse, DbErr> {
        let db = get_db();
        // 密码哈希处理
        let hashed_password = Self::hash_password(&req.password)?;
        let user = ActiveModel {
            id: Set(Uuid::new_v4()),
            username: Set(req.username),
            email: Set(req.email),
            pass_word: Set(hashed_password),
            state: Set(1),
            created_at: Set(Utc::now()),
        };

        let user: UserModel = user.insert(db).await?;

        Ok(UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
        })
    }

    fn hash_password(password: &str) -> Result<String, DbErr> {
        // 生成随机盐（使用推荐的安全盐生成方式）
        let salt = SaltString::generate(&mut OsRng);

        // 创建 Argon2 实例（使用默认安全参数）
        let argon2 = Argon2::default();

        // 生成密码哈希
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| DbErr::Custom(e.to_string()))?
            .to_string();

        Ok(password_hash)
    }


    pub async fn get_user(id: Uuid) -> Result<UserResponse, DbErr> {
        let db = get_db();
        let user = Users::find()
            .filter(users::Column::Id.eq(id))
            .one(db)
            .await?;
        
        match user {
            Some(model) => {
                Ok(UserResponse {
                    id: model.id,
                    username: model.username,
                    email: model.email,
                    created_at: model.created_at
                })
            }
            None => {
                Err(DbErr::RecordNotFound(format!("用户 {} 不存在", id)))
            }
        }
    }

    pub async fn delete_user(id: uuid::Uuid) -> Result<(), DbErr> {
        let db = get_db();
        
        let _user = Users::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!("用户 {} 不存在", id)))?;
        
        let delete_result = Users::delete_by_id(id)
            .exec(db)
            .await?;
        
        if delete_result.rows_affected == 0 {
            return Err(DbErr::Custom("删除操作未影响任何记录".into()));
        }
        
        Ok(())
    }

    pub async fn update_user(id: uuid::Uuid, update_data: UpdateUserRequest) -> Result<(), DbErr> {
        let db = get_db();
        
        let user = Users::find_by_id(id)
            .one(db)
            .await?
            .ok_or(DbErr::RecordNotFound(format!("用户 {} 不存在", id)))?;
        
        let mut active_model: users::ActiveModel = user.into();

        if let Some(username) = update_data.username {
            active_model.username = Set(username);
        }

        if let Some(email) = update_data.email {
            active_model.email = Set(email);
        }

        if let Some(state) = update_data.state {
            active_model.state = Set(state as i64);
        }
        
        let _updated_user: users::Model = active_model
            .update(db)
            .await?;
        
        Ok(())
    }
    
    pub async fn list_users(mut req: ListRequest) -> Result<ListResponse, DbErr> {
        // 获取db连接
        let db  = get_db();
        
        let mut query = Users::find();
        
        // 处理用户名过滤
        if let Some(username) = &req.username {
            // 检查是否非空字符串
            if !username.trim().is_empty() {
                query = query.filter(
                    users::Column::Username
                        .contains(username.trim()) // 去除前后空格
                );
            }
        }

        // 处理邮箱过滤
        if let Some(email) = &req.email {
            if !email.trim().is_empty() {
                query = query.filter(
                    users::Column::Email
                        .contains(email.trim().to_lowercase()) // 统一小写处理
                );
            }
        }
        
        if req.page == 0 {
            req.page = 1;
        }
        
        if req.page_size <= 0 || req.page_size > 100 {
            req.page_size = 100;
        }
        
        // 计算总记录数
        // 将query开辟新空间
        let count_query = query.clone();
        let total_result = count_query.count(db).await;
        
        let total: u64;
        match total_result {
            Ok(num) => {
                total = num;
            },
            Err(e) => return {
                Err(e)
            },
        };

        // 计算偏移量, 类型转换
        let offset: u64 = ((req.page - 1) * req.page_size) as u64;
        query = query.offset(offset);
        query = query.limit(req.page_size as u64);
        
        let query_result = query.all(db).await;
        let user_list = match query_result { 
            Ok(user_list) => {
                //初始化一个列表
                let mut list = Vec::new();
                for user in user_list {
                    list.push(UserResponse {
                        id: user.id,
                        username: user.username,
                        email: user.email,
                        created_at: user.created_at,
                    });
                }
                list
            }
            Err(e) => return Err(DbErr::Custom(e.to_string())),
        };

        // 计算总页数
        let total_pages = (total as f64 / req.page_size as f64).ceil() as u64;

        Ok(ListResponse{
            list: user_list,
            total: total as i32,
            page: req.page,
            total_page: total_pages as i32,
        })
    }
}