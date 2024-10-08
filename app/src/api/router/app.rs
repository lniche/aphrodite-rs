use crate::api::{
    controller::{account, auth, health, project},
    middleware,
};
use axum::{
    body::Body,
    http::Request,
    routing::{get, post},
    Router,
};
use tower_http::trace::TraceLayer;
use utoipa::openapi::{OpenApiBuilder, ServerBuilder};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn init() -> Router {
    // 开放
    let open = Router::new()
        .route("/login", post(auth::login))
        .route("/ping", get(health::ping));

    // 需授权
    let auth = Router::new()
        .route("/logout", get(auth::logout))
        .route("/accounts", get(account::list).post(account::create))
        .route("/accounts/:account_id", get(account::info))
        .route("/projects", get(project::list).post(project::create))
        .route("/projects/:project_id", get(project::detail))
        .layer(axum::middleware::from_fn(middleware::auth::handle));

    #[derive(OpenApi)]
    #[openapi(
        info(
            title = "Aphrodite",
            version = "1.0.0",
            description = "Aphrodite-rs Api",
        ),
        paths(
            health::ping
        ),
        tags(
            (name = "health", description = "Basic health check to see if the server is up"),
        )
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
        let openapi = builder.servers(Some(servers)).build();

        std::fs::write("openapi.json", openapi.to_pretty_json().unwrap())
            .expect("Unable to create file");

        openapi
    }

    let openapi = generate_openapi_json("http://localhost:8000".parse().unwrap());

    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
        .route("/", get(|| async { "☺ welcome to Aphrodite" }))
        .nest("/v1", open.merge(auth))
        .layer(axum::middleware::from_fn(pkg::middleware::log::handle))
        .layer(axum::middleware::from_fn(pkg::middleware::identity::handle))
        .layer(axum::middleware::from_fn(pkg::middleware::cors::handle))
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
        .layer(axum::middleware::from_fn(pkg::middleware::req_id::handle))
}
