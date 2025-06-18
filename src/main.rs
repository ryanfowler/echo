use axum::{Json, Router, body::to_bytes, extract::Request, http::Uri, response::IntoResponse};
use axum_extra::extract::Host;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64_ENGINE;
use percent_encoding::percent_decode_str;
use serde::Serialize;

#[tokio::main]
async fn main() {
    // Set up the Axum router to match any path and use echo_handler for all requests.
    let app = Router::new().fallback(echo_handler);

    // Get the port from the PORT environment variable, default to 8081 if not set.
    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    println!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Handler that echoes request details and body in a JSON response.
async fn echo_handler(host: Host, uri: Uri, req: Request) -> impl IntoResponse {
    #[derive(Debug, Serialize)]
    #[serde(untagged)]
    enum BodyField {
        Json(serde_json::Value),
        Utf8(String),
        Base64(String),
    }
    #[derive(Debug, Serialize)]
    struct Response {
        host: String,
        method: String,
        path: String,
        headers: serde_json::Map<String, serde_json::Value>,
        #[serde(skip_serializing_if = "Option::is_none")]
        query_params: Option<serde_json::Map<String, serde_json::Value>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        body: Option<BodyField>,
    }

    // Parse query parameters if present.
    let mut query_params = None;
    if let Some(query) = uri.query() {
        let mut qp = serde_json::Map::new();
        for (k, v) in form_urlencoded::parse(query.as_bytes()) {
            qp.insert(k.to_string(), v.to_string().into());
        }
        query_params = Some(qp);
    }

    // Collect all request headers into a map.
    let method = req.method().to_string();
    let mut headers = serde_json::Map::new();
    for (key, val) in req.headers() {
        headers.insert(key.to_string(), val.to_str().unwrap().into());
    }
    let path = percent_decode_str(uri.path())
        .decode_utf8_lossy()
        .to_string();

    // Read the request body as bytes (up to 1MB).
    let body = req.into_body();
    let body_bytes = to_bytes(body, 1024 * 1024).await.unwrap_or_default();

    // Determine the body representation: JSON, UTF-8 string, or otherwise.
    let body_field = if body_bytes.is_empty() {
        None
    } else if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&body_bytes) {
        Some(BodyField::Json(json))
    } else if let Ok(s) = std::str::from_utf8(&body_bytes) {
        Some(BodyField::Utf8(s.to_string()))
    } else {
        Some(BodyField::Base64(BASE64_ENGINE.encode(&body_bytes)))
    };

    Json(Response {
        host: host.0,
        method,
        path,
        headers,
        query_params,
        body: body_field,
    })
}
