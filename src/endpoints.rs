use crate::request::pagination::{ProjectSearchDelegate, ProjectSearchStream};
use crate::request::params::ProjectSearchParams;
use crate::request::response::{ApiPageResult, ApiResponse, ApiResult};
use crate::types::project::{Project, ProjectIdentifier, ProjectSearchResult};
use crate::types::version::{FileHashes, ProjectVersion};
use crate::Error;

pub static DEFAULT_API_BASE: &str = "https://api.modrinth.com/v2/";

macro_rules! endpoint {
    (
        $client:ident $method:ident,
        uri: $base:ident / $path:literal,
        $(vars: [$($var:expr),+],)?
        $(params: $params:expr,)?
        $(body: $body:expr,)?
    ) => {{
        use futures_lite::io::AsyncReadExt;

        #[allow(unused_mut)]
        let mut uri = endpoint!(@uri, $base, $path $(, [$($var),*])?);
        $(uri.set_query(Some(&serde_qs::to_string($params).unwrap()));)?

        let builder = isahc::Request::builder()
            .method(endpoint!(@str $method))
            .uri(uri.as_str());
        let request = endpoint!(@build, builder $(, $body)?)?;

        let response = $client.send_async(request).await?;
        let status = response.status();
        let mut bytes = Vec::new();

        response.into_body().read_to_end(&mut bytes).await.unwrap();

        if status != 200 {
            return Err(Error::StatusNotOk { uri, status, bytes: Box::new(bytes) });
        }

        let deser = &mut serde_json::Deserializer::from_slice(bytes.as_slice());
        let result = serde_path_to_error::deserialize(deser);

        match result {
            Ok(value) => Ok(ApiResponse { bytes, value }),
            Err(error) => Err(Error::Deserialize { uri, error, bytes: Box::new(bytes) }),
        }
    }};
    (@uri, $base:ident, $path:literal) => {
        $base.join($path).unwrap()
    };
    (@uri, $base:ident, $path:literal, [$($var:expr),+]) => {
        $base.join(&format!($path, $($var),*)).unwrap()
    };
    (@build, $builder:ident) => {
        $builder.body(())
    };
    (@build, $builder:ident, $body:expr) => {
        $builder.body(serde_json::to_string($body).unwrap())
    };
    (@str GET) => {
        "GET"
    };
    (@str POST) => {
        "POST"
    };
}

macro_rules! instantiate {
    (
        $(#[$struct_meta:meta])*
        $struct_name:ident $(<$($struct_life:lifetime),+>)? {
            $(
                $(#[$field_meta:meta])*
                $field_name:ident: $field_type:ty = $field_value:expr,
            )+
        }
    ) => {{
        $(#[$struct_meta])*
        struct $struct_name $(<$($struct_life),*>)? {
            $(
                $(#[$field_meta])*
                $field_name: $field_type,
            )*
        }

        $struct_name {
            $($field_name: $field_value,)*
        }
    }};
}

fn serialize_debug<T, S>(subject: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: std::fmt::Debug + serde::Serialize,
    S: serde::Serializer,
{
    serializer.collect_str(&format_args!("{:?}", subject))
}

pub async fn search_projects(
    client: &isahc::HttpClient,
    base: &url::Url,
    params: &ProjectSearchParams,
) -> ApiPageResult<ProjectSearchResult> {
    endpoint! {
        client GET,
        uri: base / "search",
        params: params,
    }
}

pub fn search_projects_iter<'cu, 'f>(
    client: &'cu isahc::HttpClient,
    base: &'cu url::Url,
    params: ProjectSearchParams,
) -> ProjectSearchStream<'cu, 'f> {
    ProjectSearchDelegate::new(client, base, params).into()
}

pub async fn project(
    client: &isahc::HttpClient,
    base: &url::Url,
    project_id: &ProjectIdentifier,
) -> ApiResult<Project> {
    endpoint! {
        client GET,
        uri: base / "project/{}",
        vars: [project_id],
    }
}

pub async fn projects<I>(
    client: &isahc::HttpClient,
    base: &url::Url,
    project_ids: I,
) -> ApiResult<Project>
where
    I: IntoIterator<Item = u64>,
{
    endpoint! {
        client GET,
        uri: base / "projects",
        params: &instantiate! {
            #[derive(serde::Serialize)]
            RequestParams {
                #[serde(serialize_with = "serialize_debug")]
                ids: Vec<String> = project_ids
                    .into_iter()
                    .map(base62::encode)
                    .collect(),
            }
        },
    }
}

// pub async fn project_dependencies(
//     client: &isahc::HttpClient,
//     base: &url::Url,
//     project_id: &ProjectIdentifier,
// ) -> ApiResult<ProjectDependencies> {
//     endpoint! {
//         client GET,
//         uri: base / "project/{}/dependencies",
//         vars: [project_id],
//     }
// }

pub async fn project_version_list(
    client: &isahc::HttpClient,
    base: &url::Url,
    project_id: &ProjectIdentifier,
) -> ApiResult<ProjectVersion> {
    endpoint! {
        client GET,
        uri: base / "project/{}/version",
        vars: [project_id],
    }
}

pub async fn project_version(
    client: &isahc::HttpClient,
    base: &url::Url,
    version_id: u64,
) -> ApiResult<ProjectVersion> {
    endpoint! {
        client GET,
        uri: base / "version/{}",
        vars: [base62::encode(version_id)],
    }
}

pub async fn project_versions<I>(
    client: &isahc::HttpClient,
    base: &url::Url,
    version_ids: I,
) -> ApiResult<Vec<ProjectVersion>>
where
    I: IntoIterator<Item = u64>,
{
    endpoint! {
        client GET,
        uri: base / "versions",
        params: &instantiate! {
            #[derive(serde::Serialize)]
            RequestParams {
                #[serde(serialize_with = "serialize_debug")]
                ids: Vec<String> = version_ids
                    .into_iter()
                    .map(base62::encode)
                    .collect(),
            }
        },
    }
}

pub async fn project_version_by_hash(
    client: &isahc::HttpClient,
    base: &url::Url,
    hash: &FileHashes,
) -> ApiResult<ProjectVersion> {
    let (hash, kind) = match hash {
        FileHashes {
            sha512: Some(hash), ..
        } => (hash, "sha512"),
        FileHashes {
            sha1: Some(hash), ..
        } => (hash, "sha1"),
        _ => todo!(),
    };

    endpoint! {
        client GET,
        uri: base / "version_file/{}",
        vars: [hash],
        params: &instantiate! {
            #[derive(serde::Serialize)]
            RequestParams<'a> {
                algorithm: &'a str = kind,
            }
        },
    }
}
