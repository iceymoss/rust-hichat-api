use sea_orm::ActiveValue::Set;
// 导入 SeaORM 的核心宏和类型
use sea_orm::entity::prelude::*;
use sea_orm::prelude::async_trait::async_trait;
use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};


// 定义实体模型（对应数据库表结构）
#[derive(Debug, Clone, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")] // 指定表名为 "users"
pub struct Model {
    // 主键字段，使用 UUID 类型，数据库自动生成（需 CockroachDB 支持）
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,

    // 用户名，字符串类型，非空
    #[sea_orm(column_type = "String(Some(255))")]
    pub username: String,

    // 邮箱，唯一约束，非空
    #[sea_orm(unique, column_type = "String(Some(255))")]
    pub email: String,
    
    #[sea_orm(column_type = "String(Some(255))")]
    pub pass_word: String,

    #[sea_orm(column_type = "Integer")]     // 对应数据库 INTEGER（i32）
    pub state: i64,

    // 创建时间，带时区的日期时间类型，默认值为当前时间
    pub created_at: DateTime<Utc>, // 统一使用 Utc 时间
}

// 定义实体的关联关系（此处无关联其他表，留空）
#[derive(Debug, Clone, Copy, EnumIter, DeriveRelation)]
pub enum Relation {}

// 实现 ActiveModel 的行为（自动处理时间戳等）
#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    // 插入新记录前的钩子（例如自动设置创建时间）
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            // 如果是插入操作，设置 created_at 为当前时间
            self.created_at = Set(chrono::Utc::now().into());
        }
        Ok(self)
    }
}