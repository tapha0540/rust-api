// use axum::http::{Method, header};
use tower_http::cors::CorsLayer;

pub fn get_cors() -> CorsLayer {
    // let cors = CorsLayer::new()
    //     .allow_origin(
    //         "https://example.com"
    //             .parse::<axum::http::HeaderValue>()
    //             .unwrap(),
    //     )
    //     // Allow specific HTTP verbs
    //     .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    //     // Allow specific headers sent by client (e.g. JSON content or Auth tokens)
    //     .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    return CorsLayer::permissive();
}
