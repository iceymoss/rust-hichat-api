## rust-hichat-api

### dir
```
my-web-app/
├── Cargo.toml            # 项目依赖和配置
├── Cargo.lock            # 依赖版本锁
├── src/
│   ├── main.rs           # 程序入口（启动服务器）
│   ├── lib.rs            # 库文件（可选，用于模块化）
│   ├── routes/           # HTTP 路由定义
│   │   ├── mod.rs        # 路由聚合（如挂载多个子路由）
│   │   ├── auth.rs       # 认证相关路由
│   │   └── api.rs        # API 路由
│   ├── handlers/         # 请求处理函数（Controller）
│   ├── models/           # 数据结构定义（DTO/实体）
│   ├── services/         # 业务逻辑层（数据库操作、业务规则）
│   ├── middleware/       # 自定义中间件（认证、日志等）
│   ├── config/           # 配置文件解析（环境变量、YAML/TOML）
│   └── utils/            # 工具函数（错误处理、日期格式化等）
├── migrations/           # 数据库迁移脚本（如使用 `diesel`）
├── tests/                # 集成测试
├── benches/              # 基准测试（可选）
├── examples/             # 示例代码（可选）
├── static/               # 静态文件（HTML/CSS/JS）
│   ├── index.html
│   ├── styles/
│   └── scripts/
└── .env                  # 本地环境变量（开发环境用）
```