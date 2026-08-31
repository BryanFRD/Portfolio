use std::sync::Arc;

use axum::Router;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Json, Response};
use axum::routing::{get, post};
use axum_extra::extract::cookie::CookieJar;
use serde_json::json;
use tower_http::trace::TraceLayer;
use validator::Validate;

use crate::csrf;
use crate::mailer::Mailer;
use crate::model::{ContactRequest, Project};

#[derive(Clone)]
pub struct AppState {
    pub projects: Arc<Vec<Project>>,
    pub mailer: Option<Arc<Mailer>>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/api/projects", get(projects))
        .route("/api/csrf", get(csrf_token))
        .route("/api/contact", post(contact))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn csrf_token(jar: CookieJar) -> (CookieJar, Json<serde_json::Value>) {
    let token = csrf::generate_token();
    let jar = jar.add(csrf::build_cookie(token.clone()));
    (jar, Json(json!({ "token": token })))
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}

async fn projects(State(state): State<AppState>) -> Json<Vec<Project>> {
    Json(state.projects.as_ref().clone())
}

async fn contact(
    State(state): State<AppState>,
    jar: CookieJar,
    headers: HeaderMap,
    Json(request): Json<ContactRequest>,
) -> Response {
    if !csrf::is_valid(&jar, &headers) {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "error": "invalid csrf token" })),
        )
            .into_response();
    }

    if let Err(errors) = request.validate() {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({ "errors": errors })),
        )
            .into_response();
    }

    let Some(mailer) = state.mailer else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "error": "contact is not configured" })),
        )
            .into_response();
    };

    match mailer.send_contact(&request).await {
        Ok(()) => StatusCode::ACCEPTED.into_response(),
        Err(error) => {
            tracing::error!(?error, "failed to send contact email");
            (
                StatusCode::BAD_GATEWAY,
                Json(json!({ "error": "failed to send message" })),
            )
                .into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, header};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;
    use crate::model::LocalizedText;

    fn test_router() -> Router {
        router(AppState {
            projects: Arc::new(vec![Project {
                slug: "demo".into(),
                name: "Demo".into(),
                category: LocalizedText {
                    fr: "Outillage".into(),
                    en: "Tooling".into(),
                },
                status: LocalizedText {
                    fr: "en prod".into(),
                    en: "in prod".into(),
                },
                year: 2026,
                description: LocalizedText {
                    fr: "Un projet de démo".into(),
                    en: "A demo project".into(),
                },
                technologies: vec!["Rust".into()],
                featured: true,
                repository_url: None,
                live_url: None,
                image_url: None,
                logo_url: None,
            }]),
            mailer: None,
        })
    }

    async fn body_json(response: Response) -> serde_json::Value {
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        serde_json::from_slice(&bytes).unwrap()
    }

    async fn csrf_pair(router: &Router) -> (String, String) {
        let response = router
            .clone()
            .oneshot(Request::get("/api/csrf").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let cookie = response
            .headers()
            .get(header::SET_COOKIE)
            .unwrap()
            .to_str()
            .unwrap()
            .split(';')
            .next()
            .unwrap()
            .to_owned();
        let json = body_json(response.into_response()).await;
        (cookie, json["token"].as_str().unwrap().to_owned())
    }

    fn contact_request(cookie: &str, token: &str, payload: &serde_json::Value) -> Request<Body> {
        Request::post("/api/contact")
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::COOKIE, cookie)
            .header("x-csrf-token", token)
            .body(Body::from(payload.to_string()))
            .unwrap()
    }

    #[tokio::test]
    async fn projects_returns_the_catalog() {
        let response = test_router()
            .oneshot(Request::get("/api/projects").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let json = body_json(response.into_response()).await;
        assert_eq!(json[0]["slug"], "demo");
        assert_eq!(json[0]["technologies"][0], "Rust");
    }

    #[tokio::test]
    async fn csrf_cookie_and_body_token_match() {
        let router = test_router();
        let (cookie, token) = csrf_pair(&router).await;

        assert_eq!(cookie, format!("csrf={token}"));
        assert_eq!(token.len(), 64);
    }

    #[tokio::test]
    async fn contact_without_csrf_token_returns_403() {
        let payload = json!({
            "name": "Jane Doe",
            "email": "jane@example.com",
            "message": "Bonjour, je souhaite discuter d'un projet."
        });
        let response = test_router()
            .oneshot(
                Request::post("/api/contact")
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(payload.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn contact_with_mismatched_csrf_token_returns_403() {
        let router = test_router();
        let (cookie, _) = csrf_pair(&router).await;
        let payload = json!({
            "name": "Jane Doe",
            "email": "jane@example.com",
            "message": "Bonjour, je souhaite discuter d'un projet."
        });
        let response = router
            .oneshot(contact_request(&cookie, "not-the-token", &payload))
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn contact_rejects_an_invalid_payload() {
        let router = test_router();
        let (cookie, token) = csrf_pair(&router).await;
        let payload = json!({ "name": "", "email": "not-an-email", "message": "short" });
        let response = router
            .oneshot(contact_request(&cookie, &token, &payload))
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let json = body_json(response.into_response()).await;
        let errors = json["errors"].as_object().unwrap();
        assert!(errors.contains_key("name"));
        assert!(errors.contains_key("email"));
        assert!(errors.contains_key("message"));
    }

    #[tokio::test]
    async fn contact_without_smtp_configuration_returns_503() {
        let router = test_router();
        let (cookie, token) = csrf_pair(&router).await;
        let payload = json!({
            "name": "Jane Doe",
            "email": "jane@example.com",
            "message": "Bonjour, je souhaite discuter d'un projet."
        });
        let response = router
            .oneshot(contact_request(&cookie, &token, &payload))
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
