// Cargo.toml 依赖配置：
// [dependencies]
// redis = { version = "0.23", features = ["tokio-comp"] }
// tokio = { version = "1.0", features = ["full"] }


use std::error::Error;
use rust_hichat_api::config::redis::{init_rdb, get_conn};
use deadpool_redis::redis::AsyncCommands;


// tokio 是运行时
#[tokio::main] // 启用 Tokio 异步运行时
async fn main() -> Result<(), Box<dyn Error>> {
    init_rdb().await?;
    
    // 正确方式：用 ? 解包获取 Connection
    let mut conn = get_conn().await?;
    
    // 执行 SET 命令
    // conn.set("my_key", "hello_redis").await?;
    // 显式告诉编译器返回值类型是 `()`
    let _: () = conn.set("my_key", "hello").await?;
    println!("Key set successfully.");

    // 执行 GET 命令
    let value: String = conn.get("my_key").await?;
    println!("Value of 'my_key': {}", value);

    Ok(())
}
