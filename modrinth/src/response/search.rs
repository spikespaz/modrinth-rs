use crate::base62::Base62;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResults {
    pub hits: Vec<ProjectResult>,
    pub offset: usize,
    pub limit: usize,
    pub total_hits: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectResult {
    pub project_id: Base62,
    pub project_type: String,
    pub slug: Option<String>,
    pub author: String,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub versions: Vec<String>,
    // Should `downloads` and `follows`be a usize but the API returns -1 sometimes
    // Reference:
    // > `labrinth::models::projects::Project` and
    // > `labrinth::database::models::project_item::Project`
    pub downloads: isize,
    pub follows: isize,
    pub icon_url: String,
    pub date_created: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
    pub license: String,
    pub client_side: String,
    pub server_side: String,
    pub gallery: Vec<String>,
}
