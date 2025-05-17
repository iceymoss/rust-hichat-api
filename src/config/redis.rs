// Cargo.toml 依赖配置：
// [dependencies]
// redis = { version = "0.24", features = ["tokio-comp"] }
// deadpool-redis = "0.12"
// tokio = { version = "1.0", features = ["full"] }

use deadpool_redis::{Config, Pool, Runtime};
use redis::{AsyncCommands, Connection};
use std::error::Error;
use std::sync::OnceLock;
use sea_orm::{DatabaseConnection, DbErr};

static RDB_CONN: OnceLock<deadpool_redis::Connection> = OnceLock::new();


async fn connect_rdb() -> Result<(), Box<dyn Error>> {
    // 创建连接池配置
    let cfg = Config::from_url("redis://localhost:6379");

    // 构建连接池（默认最大连接数：CPU核心数×4）
    let pool: Pool = cfg.create_pool(Some(Runtime::Tokio1))?;

    // 从池中获取连接（自动管理生命周期）
    let pool_conn = pool.get().await?;

    let save_result = RDB_CONN.set(pool_conn);
    
    match save_result {
        Ok(_) => {
            println!("✅ 成功初始化数据库连接");
            Ok(())
        },
        Err(_) => return Err("conn redis failed".into()),
    }
}

/// 获取全局数据库连接
pub fn get_rdb() -> &'static deadpool_redis::Connection {
    RDB_CONN.get().expect("redis connection not initialized")
}


/// 异步初始化函数
pub async fn init_rdb() -> Result<(), DbErr> {
    if RDB_CONN.get().is_none() {
        connect_rdb().await;
    }
    Ok(())
}
