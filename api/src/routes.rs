use std::sync::Arc;

use axum::Router;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::{IntoResponse, Json, Response};
use axum::routing::{get, post};
use axum_extra::extract::cookie::CookieJar;
use serde_json::json;
use tower_http::trace::TraceLayer;
use validator::Validate;

use crate::csrf;
use crate::mailer::Mailer;
use crate::model::{ContactRequest, Project};
use crate::ratelimit::{self, RateLimiter};

#[derive(Clone)]
pub struct AppState {
    pub projects: Arc<ProjectsPayload>,
    pub mailer: Option<Arc<Mailer>>,
    pub limiter: Arc<RateLimiter>,
}

/// Le catalogue est embarque dans le binaire : il ne change qu'entre deux
/// deploiements. On serialise une fois au demarrage et on derive un ETag du
/// contenu, ce qui permet de repondre 304 sans re-serialiser a chaque appel.
pub struct ProjectsPayload {
    body: String,
    etag: String,
}

impl ProjectsPayload {
    pub fn new(projects: &[Project]) -> Self {
        let body = serde_json::to_string(projects).expect("projects are serializable");
        let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
        for byte in body.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        Self {
            etag: format!("\"{hash:016x}\""),
            body,
        }
    }
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

/// Le jeton n'a de sens qu'appaire au cookie pose par la meme reponse : un
/// cache intermediaire qui reservirait un ancien jeton ferait echouer l'envoi
/// du formulaire en 403.
async fn csrf_token(jar: CookieJar) -> impl IntoResponse {
    let token = csrf::generate_token();
    let jar = jar.add(csrf::build_cookie(token.clone()));
    (
        jar,
        [(header::CACHE_CONTROL, "no-store")],
        Json(json!({ "token": token })),
    )
}

async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}

const PROJECTS_CACHE_CONTROL: &str = "public, max-age=300, stale-while-revalidate=86400";

async fn projects(State(state): State<AppState>, headers: HeaderMap) -> Response {
    let payload = state.projects.as_ref();
    let known = headers
        .get(header::IF_NONE_MATCH)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.split(',').any(|tag| tag.trim() == payload.etag));

    let common = [
        (header::ETAG, payload.etag.as_str()),
        (header::CACHE_CONTROL, PROJECTS_CACHE_CONTROL),
    ];

    if known {
        return (StatusCode::NOT_MODIFIED, common).into_response();
    }

    (
        common,
        [(header::CONTENT_TYPE, "application/json")],
        payload.body.clone(),
    )
        .into_response()
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

    // Après le CSRF, qui écarte les appels croisés, mais avant tout envoi :
    // l'accusé de réception part vers une adresse fournie par l'appelant.
    if let Some(ip) = ratelimit::client_ip(&headers)
        && !state.limiter.allow(&ip)
    {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            Json(json!({ "error": "too many messages, try again later" })),
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
        Ok(()) => {
            // L'accusé est un confort : son échec ne doit pas faire croire au
            // visiteur que son message s'est perdu, il est bien arrivé.
            if let Err(error) = mailer.send_acknowledgement(&request).await {
                tracing::warn!(?error, "failed to send acknowledgement to the sender");
            }
            StatusCode::ACCEPTED.into_response()
        }
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
            projects: Arc::new(ProjectsPayload::new(&[Project {
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
            }])),
            mailer: None,
            limiter: Arc::new(RateLimiter::new()),
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

    fn contact_request_from(
        ip: &str,
        cookie: &str,
        token: &str,
        payload: &serde_json::Value,
    ) -> Request<Body> {
        Request::post("/api/contact")
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::COOKIE, cookie)
            .header("x-csrf-token", token)
            .header("x-real-ip", ip)
            .body(Body::from(payload.to_string()))
            .unwrap()
    }

    /// Le quota précède l'envoi : sans mailer les appels autorisés répondent
    /// 503, ce qui suffit à distinguer le refus pour quota du reste.
    #[tokio::test]
    async fn contact_refuses_once_the_ip_quota_is_spent() {
        let router = test_router();
        let payload = json!({
            "name": "Jane Doe",
            "email": "jane@example.com",
            "message": "Bonjour, je souhaite discuter d'un projet."
        });

        for _ in 0..3 {
            let (cookie, token) = csrf_pair(&router).await;
            let response = router
                .clone()
                .oneshot(contact_request_from(
                    "203.0.113.9",
                    &cookie,
                    &token,
                    &payload,
                ))
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        }

        let (cookie, token) = csrf_pair(&router).await;
        let response = router
            .clone()
            .oneshot(contact_request_from(
                "203.0.113.9",
                &cookie,
                &token,
                &payload,
            ))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::TOO_MANY_REQUESTS);

        // Une autre adresse conserve son propre quota.
        let (cookie, token) = csrf_pair(&router).await;
        let response = router
            .clone()
            .oneshot(contact_request_from(
                "203.0.113.10",
                &cookie,
                &token,
                &payload,
            ))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn projects_returns_the_catalog() {
        let response = test_router()
            .oneshot(Request::get("/api/projects").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response
                .headers()
                .get(header::CACHE_CONTROL)
                .and_then(|value| value.to_str().ok()),
            Some(PROJECTS_CACHE_CONTROL)
        );
        assert!(response.headers().contains_key(header::ETAG));
        let json = body_json(response.into_response()).await;
        assert_eq!(json[0]["slug"], "demo");
        assert_eq!(json[0]["technologies"][0], "Rust");
    }

    #[tokio::test]
    async fn projects_answers_304_when_the_etag_still_matches() {
        let router = test_router();
        let first = router
            .clone()
            .oneshot(Request::get("/api/projects").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let etag = first
            .headers()
            .get(header::ETAG)
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();

        let cached = router
            .clone()
            .oneshot(
                Request::get("/api/projects")
                    .header(header::IF_NONE_MATCH, &etag)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(cached.status(), StatusCode::NOT_MODIFIED);
        assert!(
            cached
                .into_body()
                .collect()
                .await
                .unwrap()
                .to_bytes()
                .is_empty()
        );

        let stale = router
            .oneshot(
                Request::get("/api/projects")
                    .header(header::IF_NONE_MATCH, "\"outdated\"")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(stale.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn csrf_cookie_and_body_token_match() {
        let router = test_router();
        let (cookie, token) = csrf_pair(&router).await;

        assert_eq!(cookie, format!("csrf={token}"));
        assert_eq!(token.len(), 64);
    }

    #[tokio::test]
    async fn csrf_token_is_never_cached() {
        let response = test_router()
            .oneshot(Request::get("/api/csrf").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(
            response
                .headers()
                .get(header::CACHE_CONTROL)
                .and_then(|value| value.to_str().ok()),
            Some("no-store")
        );
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
