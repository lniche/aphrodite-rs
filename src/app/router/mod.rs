pub mod route;
use crate::app::api::auth::{LoginReq, LoginResp, SendVerifyCodeReq};
use crate::app::api::user::{GetUserResp, UpdateUserReq};
use crate::app::{
    api::{auth, health, user},
    middleware,
};
use crate::pkg::result::status::Resp;
use axum::{routing::get, Router};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::openapi::{ComponentsBuilder, OpenApiBuilder, ServerBuilder};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn init() -> Router {
    let open = Router::new().merge(route::open());

    let auth = Router::new()
        .layer(axum::middleware::from_fn(middleware::auth::handle))
        .merge(route::auth());

    #[derive(OpenApi)]
    #[openapi(
        info(
            title = "Aphrodite API",
            description = "API Description",
            version = "1.0.0",
        ),
        paths(auth::login, auth::logout,auth::send_verify_code,user::info,user::update,user::delete),
        // components(schemas(ReqLogin, RespLogin))
    )]
    struct ApiDoc;

    pub fn generate_openapi_json() -> utoipa::openapi::OpenApi {
        let dev_address = "http://127.0.0.1:8000".to_string();
        let test_address = "http://test.aphrodite.com".to_string();

        let servers = vec![
            ServerBuilder::new()
                .url(dev_address)
                .description(Some("Development Environment"))
                .build(),
            ServerBuilder::new()
                .url(test_address)
                .description(Some("Test Environment"))
                .build(),
        ];

        let builder: OpenApiBuilder = ApiDoc::openapi().into();
        let openapi = builder
            .servers(Some(servers))
            .components(Some(
                ComponentsBuilder::new()
                    .schema_from::<LoginReq>()
                    .schema_from::<LoginResp>()
                    .schema_from::<SendVerifyCodeReq>()
                    .schema_from::<UpdateUserReq>()
                    .schema_from::<GetUserResp>()
                    .schema_from::<Resp>()
                    .security_scheme(
                        "bearer_auth",
                        SecurityScheme::Http(
                            HttpBuilder::new().scheme(HttpAuthScheme::Bearer).build(),
                        ),
                    )
                    .build(),
            ))
            .build();

        std::fs::write("./docs/openapi.json", openapi.to_pretty_json().unwrap())
            .expect("Unable to create file");

        openapi
    }

    let openapi = generate_openapi_json();

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
        .route("/", get(health::root))
        .route("/ping", get(health::ping))
        .nest("/v1", open.merge(auth))
        .layer(axum::middleware::from_fn(
            crate::pkg::middleware::log::handle,
        ))
        .layer(axum::middleware::from_fn(
            crate::pkg::middleware::identity::handle,
        ))
        .layer(axum::middleware::from_fn(
            crate::pkg::middleware::cors::handle,
        ))
        .layer(axum::middleware::from_fn(
            crate::pkg::middleware::trace::handle,
        ))
}
