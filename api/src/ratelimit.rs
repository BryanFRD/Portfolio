use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use axum::http::HeaderMap;

/// Envois autorisés par adresse IP sur la durée de la fenêtre.
const MAX_HITS: usize = 3;
const WINDOW: Duration = Duration::from_secs(3600);

/// Compteur glissant en mémoire, suffisant pour un formulaire de contact.
///
/// Le portfolio tourne en un seul exemplaire : un état partagé entre replicas
/// n'apporterait rien aujourd'hui, et le compteur repart à zéro au redémarrage
/// du pod. Ce qu'il empêche, c'est l'envoi automatisé en série vers des
/// adresses tierces, pas un attaquant patient.
pub struct RateLimiter {
    hits: Mutex<HashMap<String, Vec<Instant>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            hits: Mutex::new(HashMap::new()),
        }
    }

    /// Enregistre une tentative et indique si elle est autorisée.
    pub fn allow(&self, key: &str) -> bool {
        let now = Instant::now();
        let mut hits = self.hits.lock().expect("rate limiter mutex poisoned");

        // Purge globale : sans elle, la table grossirait indéfiniment au fil
        // des adresses vues une seule fois.
        hits.retain(|_, seen| {
            seen.retain(|at| now.duration_since(*at) < WINDOW);
            !seen.is_empty()
        });

        let seen = hits.entry(key.to_owned()).or_default();
        if seen.len() >= MAX_HITS {
            return false;
        }
        seen.push(now);
        true
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

/// Adresse du client telle que la rapporte le proxy.
///
/// Traefik ajoute son observation à la fin de `X-Forwarded-For`, donc le
/// dernier élément est le seul que le client ne puisse pas forger : une chaîne
/// envoyée par l'appelant se retrouve devant celle du proxy. On préfère
/// `X-Real-Ip` quand il est présent, pour la même raison.
///
/// Sans proxy devant, aucun en-tête n'est posé et la limitation ne s'applique
/// pas : c'est le cas en développement et dans les tests.
pub fn client_ip(headers: &HeaderMap) -> Option<String> {
    if let Some(ip) = headers.get("x-real-ip").and_then(|v| v.to_str().ok()) {
        let ip = ip.trim();
        if !ip.is_empty() {
            return Some(ip.to_owned());
        }
    }

    headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.rsplit(',').next())
        .map(str::trim)
        .filter(|ip| !ip.is_empty())
        .map(str::to_owned)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn headers(pairs: &[(&str, &str)]) -> HeaderMap {
        let mut headers = HeaderMap::new();
        for (name, value) in pairs {
            let name = axum::http::HeaderName::from_bytes(name.as_bytes()).unwrap();
            headers.insert(name, value.parse().unwrap());
        }
        headers
    }

    #[test]
    fn allows_up_to_the_limit_then_refuses() {
        let limiter = RateLimiter::new();
        for _ in 0..MAX_HITS {
            assert!(limiter.allow("10.0.0.1"));
        }
        assert!(!limiter.allow("10.0.0.1"));
    }

    #[test]
    fn counts_each_address_separately() {
        let limiter = RateLimiter::new();
        for _ in 0..MAX_HITS {
            assert!(limiter.allow("10.0.0.1"));
        }
        assert!(limiter.allow("10.0.0.2"));
    }

    #[test]
    fn prefers_x_real_ip() {
        let headers = headers(&[
            ("x-real-ip", "203.0.113.7"),
            ("x-forwarded-for", "198.51.100.1"),
        ]);
        assert_eq!(client_ip(&headers).as_deref(), Some("203.0.113.7"));
    }

    #[test]
    fn keeps_the_last_forwarded_hop_which_the_client_cannot_forge() {
        let headers = headers(&[("x-forwarded-for", "1.2.3.4, 203.0.113.7")]);
        assert_eq!(client_ip(&headers).as_deref(), Some("203.0.113.7"));
    }

    #[test]
    fn returns_none_without_a_proxy() {
        assert_eq!(client_ip(&HeaderMap::new()), None);
    }
}
