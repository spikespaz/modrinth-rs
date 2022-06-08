use std::hash::Hash;

use chrono::{DateTime, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_with::SerializeDisplay;
use strum::EnumString;

use crate::base62::Base62;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Display, SerializeDisplay)]
pub enum ProjectIdentifier {
    Id(Base62),
    Slug(String),
}

impl ProjectIdentifier {
    pub fn id(other: Base62) -> Self {
        Self::from(other)
    }

    pub fn slug<S>(other: S) -> Self
    where
        S: AsRef<str>,
    {
        Self::from(other)
    }
}

impl From<Base62> for ProjectIdentifier {
    fn from(other: Base62) -> Self {
        Self::Id(other)
    }
}

impl<S> From<S> for ProjectIdentifier
where
    S: AsRef<str>,
{
    fn from(other: S) -> Self {
        Self::Slug(other.as_ref().to_owned())
    }
}

/// The API specification states that the fields `project_type`, `client_side`, and `server_side` are required,
/// and by implication, that it must match one of the variants. However, this has been seen to not be the case.
/// There is [`ProjectType::Unknown`] and [`SideSupport::Unknown`] to mitigate this issue.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Project {
    pub id: Base62,
    pub slug: Option<String>,
    pub project_type: ProjectType,
    pub team: Base62,
    pub title: String,
    pub description: String,
    pub body: String,
    #[deprecated]
    pub body_url: Option<String>,
    pub published: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub status: ProjectStatus,
    pub moderator_message: Option<ModeratorMessage>,
    pub license: ProjectLicense,
    pub client_side: SideSupport,
    pub server_side: SideSupport,
    pub downloads: usize,
    pub followers: usize,
    pub categories: Vec<String>,
    pub versions: Vec<Base62>,
    pub icon_url: Option<String>,
    pub issues_url: Option<String>,
    pub source_url: Option<String>,
    pub wiki_url: Option<String>,
    pub discord_url: Option<String>,
    pub donation_urls: Option<Vec<DonationLink>>,
    pub gallery: Vec<GalleryItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectSearchResult {
    pub project_id: Base62,
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
    pub date_created: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
    pub license: String,
    pub client_side: SideSupport,
    pub server_side: SideSupport,
    pub gallery: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumString, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ProjectType {
    Mod,
    Modpack,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumString, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    Approved,
    Archived,
    Rejected,
    Draft,
    Unlisted,
    Processing,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModeratorMessage {
    pub message: String,
    pub body: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectLicense {
    pub id: String,
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumString, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SideSupport {
    Required,
    Optional,
    Unsupported,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DonationLink {
    pub id: String,
    pub platform: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GalleryItem {
    pub url: String,
    pub featured: bool,
    pub title: Option<String>,
    pub description: Option<String>,
    pub created: DateTime<Utc>,
}
