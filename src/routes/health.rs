use axum::{routing::get, Router};

pub(crate) fn router() -> Router {
    async fn handler() -> &'static str {
        "healthy"
    }

    Router::new().route("/_health", get(handler))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn health() {
        let api = router();

        let req = Request::builder()
            .uri("/_health")
            .body(Body::empty())
            .unwrap();

        let response = api.oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"healthy");
    }
}
