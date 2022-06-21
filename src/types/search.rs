use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use time::OffsetDateTime;

use super::project::{ProjectType, SideSupport};
use crate::serde_with::Base62;

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectSearchResult {
    #[serde_as(as = "Base62<u64>")]
    pub project_id: u64,
    pub project_type: ProjectType,
    pub slug: Option<String>,
    pub author: String,
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub versions: Vec<String>,
    pub latest_version: Option<String>,
    // The next two should be `usize` but the API seems to be returning `-1`.
    // Reference:
    // > `labrinth::models::projects::Project` and
    // > `labrinth::database::models::project_item::Project`
    pub downloads: isize,
    pub follows: isize,
    pub icon_url: String,
    #[serde(with = "time::serde::rfc3339")]
    pub date_created: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub date_modified: OffsetDateTime,
    pub license: String,
    pub client_side: SideSupport,
    pub server_side: SideSupport,
    pub gallery: Vec<String>,
}
