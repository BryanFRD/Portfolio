use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub slug: String,
    pub name: String,
    pub category: String,
    pub status: String,
    pub year: u16,
    pub description: String,
    pub technologies: Vec<String>,
    #[serde(default)]
    pub featured: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct ContactRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 10, max = 5000))]
    pub message: String,
}

pub fn load_projects() -> Vec<Project> {
    serde_json::from_str(include_str!("../assets/projects.json"))
        .expect("assets/projects.json is invalid")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_projects_catalog_parses() {
        let projects = load_projects();
        assert!(!projects.is_empty());
        assert!(projects.iter().all(|project| !project.slug.is_empty()));
    }

    #[test]
    fn project_serializes_to_camel_case_and_omits_empty_links() {
        let project = Project {
            slug: "demo".into(),
            name: "Demo".into(),
            category: "Outillage".into(),
            status: "en prod".into(),
            year: 2026,
            description: "A demo project".into(),
            technologies: vec!["Rust".into()],
            featured: false,
            repository_url: Some("https://example.com/repo".into()),
            live_url: None,
            image_url: None,
            logo_url: None,
        };

        let json = serde_json::to_value(&project).unwrap();

        assert_eq!(json["repositoryUrl"], "https://example.com/repo");
        assert!(json.get("liveUrl").is_none());
        assert!(json.get("imageUrl").is_none());
        assert!(json.get("logoUrl").is_none());
    }
}
