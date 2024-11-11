# Aphrodite Axum API Scaffold

[English](README.md) | [简体中文](README-zh.md)

Aphrodite is a template project developed based on [api-tpl-rs](https://github.com/shenghui0779/yiirs), which aims to help developers get started quickly and deeply understand the use process of the framework. The project provides comprehensive sample code and configuration, covering common development scenarios, for easy learning and practice. In addition, Aphrodite also includes container deployment templates, making the project easy to deploy and manage in modern cloud environments, helping developers to efficiently build and release applications.

## Tech Stack

| Technology                                                      | Description                                                                                   |
| --------------------------------------------------------------- | --------------------------------------------------------------------------------------------- |
| [tokio](https://github.com/tokio-rs/tokio)                      | Asynchronous runtime, supporting multiple asynchronous functions and full features            |
| [clap](https://github.com/clap-rs/clap)                         | Command line parameter parsing library, supporting derivative macros                          |
| [thiserror](https://github.com/dtolnay/thiserror)               | Error handling library, providing concise error definitions                                   |
| [anyhow](https://github.com/dtolnay/anyhow)                     | Flexible error handling library, suitable for simplifying error propagation                   |
| [base64](https://crates.io/crates/base64)                       | Base64 encoding and decoding library                                                          |
| [time](https://crates.io/crates/time)                           | Time processing library, supporting macros, local time zone, formatting and parsing functions |
| [serde](https://serde.rs/)                                      | Data serialization and deserialization library, support for derivative macros                 |
| [serde_json](https://crates.io/crates/serde_json)               | JSON data serialization and deserialization library                                           |
| [tracing](https://github.com/tokio-rs/tracing)                  | Asynchronous application logging framework                                                    |
| [tracing-subscriber](https://github.com/tokio-rs/tracing)       | Log subscriber, support JSON format                                                           |
| [tracing-appender](https://crates.io/crates/tracing-appender)   | Log appender, used to write logs to files                                                     |
| [hyper](https://github.com/hyperium/hyper)                      | High-performance HTTP implementation                                                          |
| [http](https://crates.io/crates/http)                           | HTTP message and request/response processing library                                          |
| [http-body](https://crates.io/crates/http-body)                 | HTTP request and response body processing library                                             |
| [http-body-util](https://crates.io/crates/http-body-util)       | HTTP request and response body utility tools                                                  |
| [tower-http](https://github.com/tower-rs/tower-http)            | HTTP-related middleware based on Tower                                                        |
| [axum](https://github.com/tokio-rs/axum)                        | Asynchronous web framework based on Tokio                                                     |
| [axum-extra](https://crates.io/crates/axum-extra)               | Axum's additional functions and extension library                                             |
| [nanoid](https://crates.io/crates/nanoid)                       | Library for generating unique IDs                                                             |
| [validator](https://github.com/Keats/validator)                 | Data validation library, support for derivative macros                                        |
| [sea-orm](https://github.com/SeaQL/sea-orm)                     | ORM framework, supports MySQL and other databases                                             |
| [redis](https://github.com/redis/redis-rs)                      | Redis client library, supports cluster and R2D2 connection pool                               |
| [utoipa](https://github.com/utahta/utoipa)                      | API documentation generation tool                                                             |
| [utoipa-swagger-ui](https://crates.io/crates/utoipa-swagger-ui) | Swagger UI integration based on Utoipa                                                        |
| [rand](https://crates.io/crates/rand)                           | Random number generation library                                                              |
| [const-hex](https://crates.io/crates/const-hex)                 | Constant hexadecimal string conversion library                                                |
| [openssl](https://crates.io/crates/openssl)                     | OpenSSL Binding, support for encryption and decryption                                        |
| [digest](https://crates.io/crates/digest)                       | Message digest algorithm library                                                              |
| [md-5](https://crates.io/crates/md5)                            | MD5 hash library                                                                              |
| [sha1](https://crates.io/crates/sha1)                           | SHA-1 hash library                                                                            |
| [sha2](https://crates.io/crates/sha2)                           | SHA-2 hash library                                                                            |
| [hmac](https://crates.io/crates/hmac)                           | HMAC (Hash Message Authentication Code) library                                               |
| [config](https://github.com/mehcode/config-rs)                  | Configuration management library, support for multiple formats                                |
| [chrono](https://crates.io/crates/chrono)                       | Date and time library                                                                         |
| [mobc](https://crates.io/crates/mobc)                           | Connection pool library, support for asynchronous operations                                  |
| [r2d2](https://crates.io/crates/r2d2)                           | Connection pool library, supports synchronous operations                                      |

## Features

- **User authentication and authorization**: Provides basic user login and permission authorization functions.

- **Distributed lock**: Distributed lock based on Redis to ensure resource security in a distributed environment.

- **Middleware support**: Built-in commonly used middleware, including authentication, request logs, cross-domain processing, etc.

- **Unified output format**: Provides a simple and easy-to-use API Result unified output method, standardizes the API response format, and improves interface consistency.

- **API modular design**: Supports modular API design, easy to expand and maintain.

- **Swagger document integration**: Automatically generates API documents for front-end development and testing.

## Structure

```
.
├── bin/ # executable scripts
├── config/ # configuration files
├── database/ # database related
├── deploy/ # deployment related files
├── docs/ # project documentation
├── src/ # core directory
│ ├── app/ # application core code
│ ├── pkg/ # public modules
├── storage/ # file storage
├── tests/ # test files
└── README.md # project description
```

## Run Local

```bash
# 1. Clone the project code base
git clone https://github.com/lniche/aphrodite-rs.git
cd aphrodite-rs

# 2. Configuration file
cd config
mv config.toml.example config.toml

# 3. Handle dependencies
# Make sure you have installed Rust and Cargo environment
cargo build

# 4. Initialize the database
database/init.sql

# 5. Start the service
cargo run -- serve
```

## Repo Activity

![Alt](https://repobeats.axiom.co/api/embed/7d3f9b2c6f3ee0be57460b614334ff2739f36b92.svg "Repobeats analytics image")

## Contribution

If you have any suggestions or ideas, please create an issue or submit a Pull Request directly.

1. Fork this repository.
2. Create a new branch: git checkout -b feature/your-feature
3. Commit your changes: git commit -m 'Add new feature'
4. Push to your branch: git push origin feature/your-feature
5. Submit a Pull Request.

## License

This project is licensed under the MIT License.

## Acknowledgements

Special thanks to all contributors and supporters, your help is essential to us!
