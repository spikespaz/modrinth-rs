use std::hash::Hash;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::EnumString;

use crate::base62::Base62Uint;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProjectVersion {
    pub id: Base62Uint,
    pub project_id: Base62Uint,
    pub author_id: Base62Uint,
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
#[serde(deny_unknown_fields)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum VersionType {
    Alpha,
    Beta,
    Release,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VersionFile {
    pub hashes: FileHashes,
    pub url: String,
    pub filename: String,
    pub primary: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileHashes {
    pub sha512: Option<String>,
    pub sha1: Option<String>,
}

impl FileHashes {
    pub fn sha512<S>(hash: S) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            sha512: Some(hash.as_ref().to_owned()),
            sha1: None,
        }
    }

    pub fn sha1<S>(hash: S) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            sha512: None,
            sha1: Some(hash.as_ref().to_owned()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VersionDependency {
    pub version_id: Option<String>,
    pub project_id: Option<Base62Uint>,
    pub dependency_type: DependencyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumString, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum DependencyType {
    Required,
    Optional,
    Incompatible,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumString, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum LoaderSupport {
    Fabric,
    Forge,
}
