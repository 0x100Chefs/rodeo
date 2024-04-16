use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{debug_handler, Router};

use hyper::body::Bytes;
use hyper::{Body, Client, Request, StatusCode, Uri};
use hyper::{HeaderMap, Method};

use std::fmt;
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::parsers;

#[derive(Debug, Clone, Default)]
pub struct Rodeo {
    config_file: PathBuf,
}

impl Rodeo {
    pub fn new(config_file: PathBuf) -> Self {
        Self { config_file }
    }
    pub async fn run(&self, port: u16) -> Result<(), hyper::Error> {
        // load env variables and the service configuration
        dotenv::dotenv().ok();
        let addr = SocketAddr::from(([0, 0, 0, 0], port));

        // the app state is the config file path
        let state = Rodeo {
            config_file: self.config_file.clone(),
        };

        println!(
            "
    Initializing Rodeo... 🦌

    Config loaded:
     --port={port}
     --config={0:?}
     --server-base-url=http://{server_url}
    ",
            self.config_file,
            server_url = addr
        );

        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new(
                std::env::var("RUST_LOG")
                    .unwrap_or_else(|_| "rodeo_debug=debug,tower_http=debug".into()),
            ))
            .with(tracing_subscriber::fmt::layer())
            .init(); // allow debugging in development set up

        // define cors scope as any
        let cors_layer = CorsLayer::new().allow_headers(Any).allow_methods([
            Method::GET,
            Method::POST,
            Method::DELETE,
            Method::PUT,
            Method::PATCH,
        ]).allow_origin(Any); // restrict methods

        // build our application with a route to match all HTTP verbs
        let app = Router::new()
            .route(
                "/proxy/*path", // match all request method
                post(handler)
                    .get(handler)
                    .patch(handler)
                    .put(handler)
                    .delete(handler),
            )
            .with_state(state)
            .route("/health", get(health_check))
            .layer(cors_layer)
            .fallback(handle_404);

        // run it
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
    }
}
#[derive(Debug, Default)]
struct Proxy {
    pub headers: HeaderMap,
    pub method: Method,
    pub path: Uri,
    pub body: Body,
}

// impl display
impl fmt::Display for Proxy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "headers {:#?}\nmethod: {}\npath: {}\nbody:{:#?}\n",
            self.headers, self.method, self.path, self.body
        )
    }
}

struct ServicePath;

#[allow(dead_code)]
impl ServicePath {
    // create a new service
    pub fn from(service_id: &str, service_base_url: String) -> (String, String) {
        let service_id = service_id.to_string();
        let service_base_url = service_base_url.to_string();

        (service_id, service_base_url)
    }

    // parse the url
    pub fn parse_url(path: Uri, config_path: PathBuf) -> String {
        println!("[request]::{path}");

        // // if the no file inthe path, return a 404 error
        // if path.path().is_empty() {
        //     return String::from("404");
        // }

        // // if no config file is found, return a 404 error
        // if !config_path.exists() {
        //     return String::from("404");
        // }


        // split the path to extract service ID
        let path = path.path().split('/').collect::<Vec<&str>>();

        // detect the recipient server
        let service_id = path[2];
        let resource_path = &path[3..].join("/");
        
        let service = parsers::parse_config(service_id, config_path).unwrap();
        let service_base_url = service.base_url; // SERVING THE REQUEST TO THE PROXY SERVER WOULD RETURN A 404 ERROR SINCE NO ROUTE WOULD BE MATCHED

        let request_url = format!("{service_base_url}{resource_path}");

        println!("{request_url}");
        request_url
    }
    // read the url from env
    fn from_env<'a>(key: &'a str, default: &'a str) -> std::string::String {
        std::env::var(key).unwrap_or(default.to_string())
    }
}

/// 404 handler
async fn handle_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        axum::response::Json(serde_json::json!({
        "success":false,
        "message":String::from("The requested resource does not exist on this server!"),
        })),
    )
}

/// health check
async fn health_check() -> impl IntoResponse {
    (
        StatusCode::OK,
        axum::response::Json(serde_json::json!({
        "success":true,
        "message":String::from("The server is up and running!"),
        })),
    )
}

// `Request` gives you the whole request for maximum control
#[debug_handler]
async fn handler(
    State(state): State<Rodeo>,
    path: Uri,
    method: Method,
    headers: HeaderMap,
    body: Bytes,
) -> Response<Body>  {
    // pass data to request builder
    let body = Body::from(body);
    let url = ServicePath::parse_url(path, state.config_file.clone());
    let mut req = Request::builder();

    // add the header to the built request object
    for (key, value) in headers {
        req.headers_mut().unwrap().insert(key.unwrap(), value);
    }

    let req = req.method(method).uri(&url).body(body).unwrap();
    let client = Client::new();
    let res = client.request(req).await;

    // map response to return type
    //TODO: improve error handling
    res.unwrap()
}
