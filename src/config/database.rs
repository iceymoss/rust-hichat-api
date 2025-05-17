// src/config/database.rs
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::sync::OnceLock;

// 使用 OnceLock 存储全局数据库连接
static DB_CONN: OnceLock<DatabaseConnection> = OnceLock::new();


/// 初始化全局数据库连接池
pub async fn connect_db() -> Result<(), DbErr> {
    let url = format!(
        "postgres://dba@localhost:26257/wc_upsells\
        ?sslmode=verify-full\
        &sslrootcert={ca_cert}\
        &sslkey={client_key}\
        &sslcert={client_cert}",
        ca_cert = "/home/jeff/certs/ca.crt",
        client_key = "/home/jeff/certs/client.dba.key",
        client_cert = "/home/jeff/certs/client.dba.crt"
    );

    let conn = Database::connect(&url).await?;

    DB_CONN.set(conn)
        .map_err(|_| DbErr::Conn(sea_orm::RuntimeErr::Internal("Failed to initialize global database connection".to_string())))?;

    Ok(())
}

/// 获取全局数据库连接
pub fn get_db() -> &'static DatabaseConnection {
    DB_CONN.get().expect("Database connection not initialized")
}


/// 异步初始化函数
pub async fn init_db() -> Result<(), DbErr> {
    if DB_CONN.get().is_none() {
        connect_db().await?;
    }
    Ok(())
}