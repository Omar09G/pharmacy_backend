use axum::body::Body;
use axum::http::{HeaderValue, Method, Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;

/// Simple CORS middleware that handles preflight requests and adds CORS headers to responses.
pub async fn cors_middleware(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    // If it's a preflight request, return an OK response with the appropriate headers
    if req.method() == Method::OPTIONS {
        let mut res = Response::new(Body::empty());
        let headers = res.headers_mut();
        headers.insert("access-control-allow-origin", HeaderValue::from_static("*"));
        headers.insert(
            "access-control-allow-methods",
            HeaderValue::from_static("GET,POST,PUT,PATCH,DELETE,OPTIONS"),
        );
        headers.insert(
            "access-control-allow-headers",
            HeaderValue::from_static("authorization,content-type"),
        );
        headers.insert(
            "access-control-allow-credentials",
            HeaderValue::from_static("true"),
        );
        return Ok(res);
    }

    // For non-preflight requests, call the next handler and then attach CORS headers
    let mut response = next.run(req).await;
    let headers = response.headers_mut();
    headers.insert("access-control-allow-origin", HeaderValue::from_static("*"));
    headers.insert(
        "access-control-allow-credentials",
        HeaderValue::from_static("true"),
    );
    Ok(response)
}
