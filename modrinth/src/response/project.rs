use crate::base62::Base62;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::EnumString;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Base62,
    pub slug: Option<String>,
    pub project_type: String,
    pub team: Base62,
    pub description: String,
    pub body: String,
    pub body_url: Option<String>,
    pub published: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub status: ProjectStatus,
    pub moderator_message: Option<ModeratorMessage>,
    pub license: ProjectLicense,
    pub client_side: SideType,
    pub server_side: SideType,
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

#[derive(Debug, Clone, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum ProjectStatus {
    Approved,
    Archived,
    Rejected,
    Draft,
    Unlisted,
    Processing,
    // Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModeratorMessage {
    pub message: String,
    pub body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectLicense {
    pub id: String,
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum SideType {
    Required,
    Optional,
    Unsupported,
    // Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DonationLink {
    pub id: String,
    pub platform: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GalleryItem {
    pub url: String,
    pub featured: bool,
    pub title: Option<String>,
    pub description: Option<String>,
    pub created: DateTime<Utc>,
}
