pub mod route;
use crate::app::api::auth::{LoginReq, LoginResp, SendVerifyCodeReq};
use crate::app::api::user::{GetUserResp, UpdateUserReq};
use crate::app::{
    api::{auth, health, user},
    middleware,
};
use axum::{body::Body, http::Request, routing::get, Router};
use tower_http::trace::TraceLayer;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::openapi::{ComponentsBuilder, OpenApiBuilder, ServerBuilder};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn init() -> Router {
    // 开放
    let open = Router::new().merge(route::open());

    // 需授权
    let auth = Router::new()
        .layer(axum::middleware::from_fn(middleware::auth::handle))
        .merge(route::auth());

    #[derive(OpenApi)]
    #[openapi(
        info(
            title = "Aphrodite API",
            version = "1.0.0",
            description = "API Description",
        ),
        paths(auth::login, auth::logout,auth::send_verify_code,user::info,user::update,user::delete),
        // components(schemas(ReqLogin, RespLogin))
    )]
    struct ApiDoc;

    pub fn generate_openapi_json(address: String) -> utoipa::openapi::OpenApi {
        // This is the equivalent of the following snippet annotation:
        // However we wannt grab the data from our env to generate the openapi.json file
        // servers(
        //     (url = "http://localhost:5000", description = "Local server"),
        // ),
        let server = ServerBuilder::new().url(address).build();

        // Add more servers as you wish here
        // Take note that the sever that you want to expose should be here else don't include it
        let servers = vec![server];
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

    let openapi = generate_openapi_json("http://127.0.0.1:8000".parse().unwrap());

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
        .route("/", get(health::home))
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
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                let req_id = match request
                    .headers()
                    .get("x-request-id")
                    .and_then(|value| value.to_str().ok())
                {
                    Some(v) => v.to_string(),
                    None => String::from("unknown"),
                };

                tracing::error_span!("request_id", id = req_id)
            }),
        )
        .layer(axum::middleware::from_fn(
            crate::pkg::middleware::trace::handle,
        ))
}
