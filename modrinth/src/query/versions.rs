use std::hash::Hash;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::EnumString;

use super::{get, projects::ProjectIdentifier, Result};
use crate::base62::Base62;

pub fn get_project_versions(
    identifier: &ProjectIdentifier,
    token: Option<&str>,
) -> Result<Vec<ProjectVersion>> {
    get(
        &format!("https://api.modrinth.com/v2/project/{}/version", identifier),
        token,
    )
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProjectVersion {
    pub id: Base62,
    pub project_id: Base62,
    pub author_id: Base62,
    pub featured: bool,
    pub name: String,
    pub version_number: String,
    pub changelog: Option<String>,
    #[deprecated]
    pub changelog_url: Option<String>,
    pub date_published: DateTime<Utc>,
    pub downloads: usize,
    pub version_type: VersionType,
    pub files: Vec<VersionFile>,
    pub dependencies: Vec<VersionDependency>,
    pub game_versions: Vec<String>,
    pub loaders: Vec<LoaderSupport>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum VersionType {
    Alpha,
    Beta,
    Release,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VersionFile {
    pub hashes: FileHashes,
    pub url: String,
    pub filename: String,
    pub primary: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FileHashes {
    pub sha512: Option<String>,
    pub sha1: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VersionDependency {
    pub version_id: Option<String>,
    pub project_id: Option<Base62>,
    pub dependency_type: DependencyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DependencyType {
    Required,
    Optional,
    Incompatible,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum LoaderSupport {
    Fabric,
    Forge,
}
