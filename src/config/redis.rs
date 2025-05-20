use deadpool_redis::{Config, Pool, Runtime, Connection};
use std::error::Error;
use std::sync::OnceLock;

// 全局连接池（正确方式）
static RDB_POOL: OnceLock<Pool> = OnceLock::new();

async fn connect_rdb() -> Result<(), Box<dyn Error>> {
    let cfg = Config::from_url("redis://localhost:6379");
    let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
    
    // 存储连接池而非单个连接
    RDB_POOL.set(pool)
        .map_err(|_| "Pool already initialized".into())
}

/// 获取全局连接池（正确方式）
fn get_pool() -> &'static Pool {
    RDB_POOL.get().expect("Redis pool not initialized")
}

/// 获取数据库连接（每次使用时获取新连接）
pub async fn get_conn() -> Result<Connection, Box<dyn Error>> {
    get_pool().get().await.map_err(|e| e.into())
}

/// 初始化函数
pub async fn init_rdb() -> Result<(), Box<dyn Error>> {
    if RDB_POOL.get().is_none() {
        connect_rdb().await?;
    }
    Ok(())
}
