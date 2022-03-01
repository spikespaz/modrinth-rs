use std::hash::Hash;

use chrono::{DateTime, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_with::SerializeDisplay;
use strum::EnumString;

use super::{get, Result};
use crate::base62::Base62;

pub fn get_project(identifier: &ProjectIdentifier, token: Option<&str>) -> Result<Project> {
    get(
        &format!("https://api.modrinth.com/v2/project/{}", identifier),
        token,
    )
}

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ProjectType {
    Mod,
    Modpack,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    Approved,
    Archived,
    Rejected,
    Draft,
    Unlisted,
    Processing,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ModeratorMessage {
    pub message: String,
    pub body: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProjectLicense {
    pub id: String,
    pub name: String,
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum SideSupport {
    Required,
    Optional,
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DonationLink {
    pub id: String,
    pub platform: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GalleryItem {
    pub url: String,
    pub featured: bool,
    pub title: Option<String>,
    pub description: Option<String>,
    pub created: DateTime<Utc>,
}
