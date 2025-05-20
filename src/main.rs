// 目录下的文件需要导入，并且配置一个mod.rs文件
// 并且在main中需要使用mod导入
// 然后才能使用use到某一个具体的内容上
// mod config;
// mod entities;
// mod handlers;
// mod models;
// mod routes;
// mod services;


use rust_hichat_api::routes::app_router::AppRouter;
use rust_hichat_api::routes;
use rust_hichat_api::config;
use rust_hichat_api::config::redis::init_rdb;

#[tokio::main]
async fn main() {
    // 初始化数据库
    let db_init_result = config::database::init_db().await;
    match db_init_result {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Database init failed: {}", e);
            return;
        }
    }

    init_rdb().await;
    
    
    let router = routes::user_router::user_routes(AppRouter::new().router);
    
    println!("Listening on http://0.0.0.0:3000");
    // 启动服务器
    let server_result = axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
        router
    ).await;

    match server_result {
        Ok(_) => (),
        Err(e) => eprintln!("Server error: {}", e)
    }
}
