use axum::http::HeaderMap;
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};

pub const CSRF_COOKIE: &str = "csrf";
pub const CSRF_HEADER: &str = "x-csrf-token";

pub fn generate_token() -> String {
    let bytes: [u8; 32] = rand::random();
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub fn build_cookie(token: String) -> Cookie<'static> {
    let mut cookie = Cookie::new(CSRF_COOKIE, token);
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Strict);
    cookie
}

pub fn is_valid(jar: &CookieJar, headers: &HeaderMap) -> bool {
    let cookie_token = jar.get(CSRF_COOKIE).map(Cookie::value);
    let header_token = headers
        .get(CSRF_HEADER)
        .and_then(|value| value.to_str().ok());
    matches!((cookie_token, header_token), (Some(cookie), Some(header)) if !cookie.is_empty() && cookie == header)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_tokens_are_unique_hex() {
        let first = generate_token();
        let second = generate_token();
        assert_eq!(first.len(), 64);
        assert!(first.chars().all(|c| c.is_ascii_hexdigit()));
        assert_ne!(first, second);
    }

    #[test]
    fn validation_requires_matching_cookie_and_header() {
        let token = generate_token();
        let jar = CookieJar::new().add(build_cookie(token.clone()));
        let mut headers = HeaderMap::new();

        assert!(!is_valid(&jar, &headers));

        headers.insert(CSRF_HEADER, token.parse().unwrap());
        assert!(is_valid(&jar, &headers));

        headers.insert(CSRF_HEADER, "wrong".parse().unwrap());
        assert!(!is_valid(&jar, &headers));

        assert!(!is_valid(&CookieJar::new(), &headers));
    }
}
