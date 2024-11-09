# Aphrodite Axum API Scaffold

Aphrodite 是一个基于 [api-tpl-rs](https://github.com/shenghui0779/yiirs) 开发的模板项目，旨在帮助开发者快速上手，深入理解框架的使用流程。该项目提供了全面的示例代码和配置，涵盖了常见的开发场景，以便于学习和实践。此外，Aphrodite 还包含容器部署模板，使得项目在现代云环境中能够轻松部署与管理，助力开发者高效构建和发布应用。

## 技术栈

| 技术                                                            | 说明                                           |
| --------------------------------------------------------------- | ---------------------------------------------- |
| [tokio](https://github.com/tokio-rs/tokio)                      | 异步运行时，支持多种异步功能和全特性           |
| [clap](https://github.com/clap-rs/clap)                         | 命令行参数解析库，支持衍生宏                   |
| [thiserror](https://github.com/dtolnay/thiserror)               | 错误处理库，提供简洁的错误定义                 |
| [anyhow](https://github.com/dtolnay/anyhow)                     | 灵活的错误处理库，适用于简化错误传播           |
| [base64](https://crates.io/crates/base64)                       | Base64 编码和解码库                            |
| [time](https://crates.io/crates/time)                           | 时间处理库，支持宏、本地时区、格式化和解析功能 |
| [serde](https://serde.rs/)                                      | 数据序列化和反序列化库，支持衍生宏             |
| [serde_json](https://crates.io/crates/serde_json)               | JSON 数据序列化和反序列化库                    |
| [tracing](https://github.com/tokio-rs/tracing)                  | 异步应用日志框架                               |
| [tracing-subscriber](https://github.com/tokio-rs/tracing)       | 日志订阅者，支持 JSON 格式                     |
| [tracing-appender](https://crates.io/crates/tracing-appender)   | 日志追加器，用于将日志写入文件                 |
| [hyper](https://github.com/hyperium/hyper)                      | 高性能的 HTTP 实现                             |
| [http](https://crates.io/crates/http)                           | HTTP 消息和请求/响应处理库                     |
| [http-body](https://crates.io/crates/http-body)                 | HTTP 请求和响应体处理库                        |
| [http-body-util](https://crates.io/crates/http-body-util)       | HTTP 请求体和响应体的实用工具                  |
| [tower-http](https://github.com/tower-rs/tower-http)            | 基于 Tower 的 HTTP 相关中间件                  |
| [axum](https://github.com/tokio-rs/axum)                        | 基于 Tokio 的异步 Web 框架                     |
| [axum-extra](https://crates.io/crates/axum-extra)               | Axum 的额外功能和扩展库                        |
| [nanoid](https://crates.io/crates/nanoid)                       | 用于生成唯一 ID 的库                           |
| [validator](https://github.com/Keats/validator)                 | 数据验证库，支持衍生宏                         |
| [sea-orm](https://github.com/SeaQL/sea-orm)                     | ORM 框架，支持 MySQL 和其他数据库              |
| [redis](https://github.com/redis/redis-rs)                      | Redis 客户端库，支持集群和 R2D2 连接池         |
| [utoipa](https://github.com/utahta/utoipa)                      | API 文档生成工具                               |
| [utoipa-swagger-ui](https://crates.io/crates/utoipa-swagger-ui) | 基于 Utoipa 的 Swagger UI 集成                 |
| [rand](https://crates.io/crates/rand)                           | 随机数生成库                                   |
| [const-hex](https://crates.io/crates/const-hex)                 | 常量十六进制字符串转换库                       |
| [openssl](https://crates.io/crates/openssl)                     | OpenSSL 绑定，支持加密和解密                   |
| [digest](https://crates.io/crates/digest)                       | 消息摘要算法库                                 |
| [md-5](https://crates.io/crates/md5)                            | MD5 哈希库                                     |
| [sha1](https://crates.io/crates/sha1)                           | SHA-1 哈希库                                   |
| [sha2](https://crates.io/crates/sha2)                           | SHA-2 哈希库                                   |
| [hmac](https://crates.io/crates/hmac)                           | HMAC（哈希消息认证码）库                       |
| [config](https://github.com/mehcode/config-rs)                  | 配置管理库，支持多种格式                       |
| [chrono](https://crates.io/crates/chrono)                       | 日期和时间库                                   |
| [mobc](https://crates.io/crates/mobc)                           | 连接池库，支持异步操作                         |
| [r2d2](https://crates.io/crates/r2d2)                           | 连接池库，支持同步操作                         |

## 特性

- **用户认证与授权**：提供基础的用户登录和权限授权功能。
- **分布式锁**：基于 Redis 实现的分布式锁，保证分布式环境下的资源安全。
- **中间件支持**：内置常用的中间件，包括认证、请求日志、跨域处理等。
- **统一输出格式**：提供简单易用的 API Result 统一输出方式，标准化 API 响应格式，提升接口一致性。
- **API 模块化设计**：支持模块化的 API 设计，易于扩展和维护。
- **Swagger 文档集成**：自动生成 API 文档，便于前端开发和测试。

## 目录结构

```
.
├── bin/                  # 可执行脚本
├── config/               # 配置文件
├── database/             # 数据库相关
├── deploy/               # 部署相关文件
├── docs/                 # 项目文档
├── src/                  # 核心目录
│   ├── app/              # 应用核心代码
│   ├── pkg/              # 公共模块
├── storage/              # 文件存储
├── tests/                # 测试文件
└── README.md             # 项目说明
```

## 本地运行

```bash
# 1. 克隆项目代码库
git clone https://github.com/lniche/aphrodite-rs.git
cd aphrodite-rs

# 2. 配置文件
cd config
mv config.toml.example config.toml

# 3. 处理依赖
# 确保你已经安装了 Rust 和 Cargo 环境
cargo build

# 4. 初始化数据库
database/init.sql

# 5. 启动服务
cargo run -- serve
```

## Repo Activity

![Alt](https://repobeats.axiom.co/api/embed/7d3f9b2c6f3ee0be57460b614334ff2739f36b92.svg "Repobeats analytics image")

## 贡献

如果你有任何建议或想法，欢迎创建 Issue 或直接提交 Pull Request。

1. Fork 这个仓库。
2. 创建一个新的分支：

```
git checkout -b feature/your-feature
```

3. 提交你的更改：

```
git commit -m 'Add new feature'
```

4. 推送到你的分支：

```
git push origin feature/your-feature
```

5. 提交 Pull Request。

## 许可证

该项目遵循 MIT 许可证。

## 鸣谢

特别感谢所有贡献者和支持者，您的帮助对我们至关重要！
