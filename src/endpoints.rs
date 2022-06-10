use crate::request::params::ProjectSearchParams;
use crate::types::{Project, ProjectSearchResult};
use crate::Error;
use crate::request::pagination::{ProjectSearchDelegate, ProjectSearchStream};
use crate::request::response::{ApiPageResult, ApiResponse};

pub static DEFAULT_API_BASE: &str = "https://api.modrinth.com/v2/";

macro_rules! endpoint {
    (
        $client:ident $method:ident,
        uri: $base:ident / $path:literal,
        $(vars: [$($var:ident),+],)?
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
    (@uri, $base:ident, $path:literal, [$($var:ident),+]) => {
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

pub async fn search_projects(
    client: &isahc::HttpClient,
    base: &url::Url,
    params: &ProjectSearchParams,
) -> ApiPageResult<ProjectSearchResult> {
    endpoint!{
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

//
// pub async fn get_project(&self, identifier: &ProjectIdentifier) -> surf::Result<Project> {
//     self.inner
//         .get(&format!("project/{}", identifier))
//         .recv_json()
//         .await
// }
//
// pub async fn get_project_versions(
//     &self,
//     identifier: &ProjectIdentifier,
// ) -> surf::Result<Vec<ProjectVersion>> {
//     self.inner
//         .get(&format!("version/{}", identifier))
//         .recv_json()
//         .await
// }
//
// pub async fn get_version(&self, identifier: &Base62) -> surf::Result<ProjectVersion> {
//     self.inner
//         .get(&format!("version/{}", identifier))
//         .recv_json()
//         .await
// }
//
// pub async fn get_version_by_hash(&self, hash: &FileHashes) -> surf::Result<ProjectVersion> {
//     self.inner
//         .get(&match hash {
//             FileHashes {
//                 sha512: Some(hash), ..
//             } => format!("version_file/{}?algorithm=sha512", hash),
//             FileHashes {
//                 sha1: Some(hash), ..
//             } => format!("version_file/{}?algorithm=sha1", hash),
//             _ => panic!("expected at least one field of `hash` to be `Some`"),
//         })
//         .recv_json()
//         .await
// }
