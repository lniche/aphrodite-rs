# Aphrodite Rust API 快速开发脚手架

Aphrodite 是一个基于 [api-tpl-rs](https://github.com/shenghui0779/yiirs) 开发的模板项目，旨在帮助开发者快速上手，深入理解框架的使用流程。该项目提供了全面的示例代码和配置，涵盖了常见的开发场景，以便于学习和实践。此外，Aphrodite 还包含容器部署模板，使得项目在现代云环境中能够轻松部署与管理，助力开发者高效构建和发布应用。

| 技术                                 | 说明                                         |
|-------------------------------------|--------------------------------------------|
| [tokio](https://github.com/tokio-rs/tokio)                 | 异步运行时，支持多种异步功能和全特性           |
| [clap](https://github.com/clap-rs/clap)                    | 命令行参数解析库，支持衍生宏                   |
| [thiserror](https://github.com/dtolnay/thiserror)          | 错误处理库，提供简洁的错误定义                 |
| [anyhow](https://github.com/dtolnay/anyhow)                | 灵活的错误处理库，适用于简化错误传播           |
| [base64](https://crates.io/crates/base64)                  | Base64 编码和解码库                          |
| [time](https://crates.io/crates/time)                       | 时间处理库，支持宏、本地时区、格式化和解析功能 |
| [serde](https://serde.rs/)                                  | 数据序列化和反序列化库，支持衍生宏             |
| [serde_json](https://crates.io/crates/serde_json)          | JSON 数据序列化和反序列化库                  |
| [tracing](https://github.com/tokio-rs/tracing)            | 异步应用日志框架                             |
| [tracing-subscriber](https://github.com/tokio-rs/tracing) | 日志订阅者，支持 JSON 格式                    |
| [tracing-appender](https://crates.io/crates/tracing-appender) | 日志追加器，用于将日志写入文件                |
| [hyper](https://github.com/hyperium/hyper)                | 高性能的 HTTP 实现                            |
| [http](https://crates.io/crates/http)                      | HTTP 消息和请求/响应处理库                    |
| [http-body](https://crates.io/crates/http-body)            | HTTP 请求和响应体处理库                      |
| [http-body-util](https://crates.io/crates/http-body-util)  | HTTP 请求体和响应体的实用工具                 |
| [tower-http](https://github.com/tower-rs/tower-http)      | 基于 Tower 的 HTTP 相关中间件                 |
| [axum](https://github.com/tokio-rs/axum)                  | 基于 Tokio 的异步 Web 框架                   |
| [axum-extra](https://crates.io/crates/axum-extra)          | Axum 的额外功能和扩展库                      |
| [nanoid](https://crates.io/crates/nanoid)                  | 用于生成唯一 ID 的库                          |
| [validator](https://github.com/Keats/validator)            | 数据验证库，支持衍生宏                       |
| [sea-orm](https://github.com/SeaQL/sea-orm)                | ORM 框架，支持 MySQL 和其他数据库              |
| [redis](https://github.com/redis/redis-rs)                 | Redis 客户端库，支持集群和 R2D2 连接池        |
| [utoipa](https://github.com/utahta/utoipa)                 | API 文档生成工具                             |
| [utoipa-swagger-ui](https://crates.io/crates/utoipa-swagger-ui) | 基于 Utoipa 的 Swagger UI 集成               |
| [rand](https://crates.io/crates/rand)                      | 随机数生成库                                 |
| [const-hex](https://crates.io/crates/const-hex)            | 常量十六进制字符串转换库                      |
| [openssl](https://crates.io/crates/openssl)                | OpenSSL 绑定，支持加密和解密                  |
| [digest](https://crates.io/crates/digest)                  | 消息摘要算法库                              |
| [md-5](https://crates.io/crates/md5)                       | MD5 哈希库                                   |
| [sha1](https://crates.io/crates/sha1)                      | SHA-1 哈希库                                 |
| [sha2](https://crates.io/crates/sha2)                      | SHA-2 哈希库                                 |
| [hmac](https://crates.io/crates/hmac)                      | HMAC（哈希消息认证码）库                     |
| [config](https://github.com/mehcode/config-rs)             | 配置管理库，支持多种格式                      |
| [chrono](https://crates.io/crates/chrono)                  | 日期和时间库                                 |
| [mobc](https://crates.io/crates/mobc)                      | 连接池库，支持异步操作                        |
| [r2d2](https://crates.io/crates/r2d2)                      | 连接池库，支持同步操作                        |

| 功能                  | 说明                                         |
|---------------------|--------------------------------------------|
| 登录授权功能        | 提供基础的用户登录和权限授权功能                    |
| 分布式锁            | 基于 Redis 实现的分布式锁                          |
| 中间件              | 包含认证、请求日志、跨域中间件                    |
| 实用封装            | 提供 AES、Hash、时间格式化等常用工具封装             |
| 统一输出方式        | 简单易用的 API Result 统一输出方式                  |

## 模块说明

- app => 应用模块
  - api => 应用模块
  - cmd => 应用模块
  - ent => 应用模块
- pkg => 公共模块

## 本地运行

```bash
# 数据库
deploy/db.sql

# 配置文件
mv config.toml.example config.toml

# 启动服务
cargo run -- serve
```
