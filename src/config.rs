use serde::Deserialize;
use std::{fmt::{self, Display}, path::PathBuf};

#[derive(Debug, Deserialize, Clone)]
pub struct MeiliSearchConfig {
    pub meilisearch_url: String,
    pub meilisearch_api_key: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProjectConfig {
    pub root: PathBuf,
    pub crontab: String,
    #[serde(default = "default_maxdepth")]
    pub max_depth: Option<usize>,
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
        writeln!(f, "  Max Depth: {}", self.max_depth.unwrap_or(0))?;
        writeln!(f, "  Ignore Rules: {}", 
            self.custom_ignore_rule_file.as_deref().unwrap_or("none")
        )?;
        write!(f, "  Index Hidden: {}\n  Follow Symlinks: {}", 
            self.index_hidden, 
            self.follow_symlinks
        )
    }
}

fn default_maxdepth() -> Option<usize> {
    None
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
    pub meilisearch: Option<MeiliSearchConfig>,
    pub projects: Vec<ProjectConfig>,
}

impl Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "MeiliSearch Configuration:")?;
        if let Some(meilisearch) = &self.meilisearch {
            writeln!(f, "  URL: {}", meilisearch.meilisearch_url)?;
            writeln!(f, "  API Key: **hidden**")?;
        } else {
            writeln!(f, "  MeiliSearch not configured")?;
        }

        writeln!(f, "\nProjects:")?;
        for project in &self.projects {
            writeln!(f, "{}\n", project)?;
        }

        Ok(())
    }
}

pub fn read_config(config_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_content = std::fs::read_to_string(config_path)?;
    Ok(toml::from_str(&config_content)?)
}
