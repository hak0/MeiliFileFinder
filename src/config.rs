use serde::Deserialize;
use std::{
    fmt::{self, Display},
    path::PathBuf,
};

#[derive(Debug, Deserialize, Clone)]
pub struct MeiliSearchConfig {
    pub meilisearch_url: String,
    pub meilisearch_api_key: String,
    #[serde(default = "default_meilisearch_bin_path")]
    pub meilisearch_bin_path: String,
    #[serde(default = "default_meilisearch_db_path")]
    pub meilisearch_db_path: String,
    #[serde(default = "default_meilisearch_telemetry")]
    pub meilisearch_telemetry: bool,
}

impl Display for MeiliSearchConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Meilisearch Configuration:")?;
        writeln!(f, "  URL: {}", self.meilisearch_url)?;
        writeln!(f, "  API Key: **hidden**")?;
        writeln!(f, "  Binary Path: {}", self.meilisearch_bin_path)?;
        writeln!(f, "  Database Path: {}", self.meilisearch_db_path)?;
        writeln!(f, "  Telemetry: {}", self.meilisearch_telemetry)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProjectConfig {
    pub root: PathBuf,
    pub crontab: String,
    #[serde(default = "default_maxdepth")]
    pub max_depth: usize,
    #[serde(default = "default_custom_ignore_rule_file")]
    pub custom_ignore_rule_file: Option<String>,
    #[serde(default = "default_index_hidden")]
    pub index_hidden: bool,
    #[serde(default = "default_follow_symlinks")]
    pub follow_symlinks: bool,
}

impl Display for ProjectConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Project Configuration:")?;
        writeln!(f, "  Root Directory: {:?}", self.root)?;
        writeln!(f, "  Schedule: {}", self.crontab)?;
        writeln!(f, "  Max Depth(0 for infinitive depth): {}", self.max_depth)?;
        writeln!(
            f,
            "  Extra Ignore-rules File: {}",
            self.custom_ignore_rule_file.as_deref().unwrap_or("none")
        )?;
        write!(
            f,
            "  Index Hidden File/Folders: {}\n  Follow Symlinks: {}",
            self.index_hidden, self.follow_symlinks
        )
    }
}

fn default_meilisearch_bin_path() -> String {
    "".to_string()
}
fn default_meilisearch_db_path() -> String {
    "".to_string()
}
fn default_meilisearch_telemetry() -> bool {
    true
}
fn default_maxdepth() -> usize {
    0
}
fn default_custom_ignore_rule_file() -> Option<String> {
    None
}
fn default_index_hidden() -> bool {
    false
}
fn default_follow_symlinks() -> bool {
    false
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub meilisearch: MeiliSearchConfig,
    pub projects: Vec<ProjectConfig>,
}

impl Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let meilisearch_config = &self.meilisearch;
        writeln!(f, "{}", meilisearch_config)?;

        writeln!(f, "Projects:")?;
        for project in &self.projects {
            writeln!(f, "{}", project)?;
        }
        if self.projects.is_empty() {
            writeln!(f, "No Projects\n")?;
        }

        Ok(())
    }
}

pub fn read_config(config_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_content = std::fs::read_to_string(config_path)?;
    Ok(toml::from_str(&config_content)?)
}
