use serde::Deserialize;
use std::sync::LazyLock;

#[derive(Clone, Deserialize, PartialEq)]
pub struct Domain {
    pub id: String,
    pub name: String,
    pub sector: String,
    pub tagline: String,
    pub summary: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub build: Vec<String>,
    pub replacement: String,
    pub proof: String,
    pub links: Vec<String>,
}

pub static DOMAINS: LazyLock<Vec<Domain>> = LazyLock::new(|| {
    serde_json::from_str(include_str!("domains.json")).expect("validated domain content")
});
pub fn domain(id: &str) -> &'static Domain {
    DOMAINS.iter().find(|d| d.id == id).unwrap_or(&DOMAINS[7])
}
pub fn number(id: &str) -> String {
    format!(
        "{:02}",
        DOMAINS.iter().position(|d| d.id == id).unwrap_or(0) + 1
    )
}
