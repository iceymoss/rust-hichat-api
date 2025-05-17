// Cargo.toml 依赖配置：
// [dependencies]
// redis = { version = "0.23", features = ["tokio-comp"] }
// tokio = { version = "1.0", features = ["full"] }

use redis::AsyncCommands;
use redis::Client;
use std::error::Error;

// tokio 是运行时
#[tokio::main] // 启用 Tokio 异步运行时
async fn main() -> Result<(), Box<dyn Error>> {
    // 创建 Redis 客户端
    let client = Client::open("redis://localhost:6379")?;

    // 获取异步连接
    let mut conn = client.get_async_connection().await?;

    // 执行 SET 命令
    // conn.set("my_key", "hello_redis").await?;
    // 显式告诉编译器返回值类型是 `()`
    let _: () = conn.set("my_key", "hello_redis").await?;
    println!("Key set successfully.");

    // 执行 GET 命令
    let value: String = conn.get("my_key").await?;
    println!("Value of 'my_key': {}", value);

    Ok(())
}