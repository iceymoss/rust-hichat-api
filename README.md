## rust-hichat-api

### dir
```
.
├── Cargo.lock // 相关依赖，版本锁定
├── Cargo.toml // 依赖文件
├── README.md  // 项目说明
├── benches    // 基准测试
├── migrations // 数据库迁移相关
├── src        // 项目核心
│   ├── config  //配置相关
│   │   ├── database.rs  // db配置
│   │   └── mod.rs       // 对外使用声明
│   ├── entities               // 数据库表结构相关
│   │   ├── mod.rs       // 对外使用声明
│   │   └── users.rs     // 表结构
│   ├── handlers               // api控制器
│   │   ├── mod.rs       // 对外使用声明
│   │   └── users_handlers.rs //user api 控制
│   ├── main.rs                // 程序入口
│   ├── middleware             //中间件相关
│   ├── models                 // api 请求和响应数据结构
│   │   ├── mod.rs       // 对外使用声明
│   │   └── users.rs      
│   ├── routes                 // api 路由配置
│   │   ├── app_router.rs
│   │   ├── mod.rs
│   │   └── user_router.rs
│   ├── services               // 核心业务
│   │   ├── mod.rs
│   │   └── user_service.rs
│   └── utils                  // 工具类
├── target                           // 依赖相关

```