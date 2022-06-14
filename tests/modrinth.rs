use modrinth::endpoints as e;
use modrinth::request::ProjectSearchParams;
use once_cell::sync::Lazy;

static CLIENT: Lazy<isahc::HttpClient> = Lazy::new(|| {
    isahc::HttpClient::builder()
        .max_connections_per_host(10)
        .default_header("content-type", "application/json")
        .default_header("accept", "application/json")
        .build()
        .unwrap()
});

static API_BASE: Lazy<url::Url> =
    Lazy::new(|| modrinth::endpoints::DEFAULT_API_BASE.parse().unwrap());

#[test]
fn search_projects_iter() {
    smol::block_on(async {
        use smol::pin;
        use smol::stream::StreamExt;

        let params = ProjectSearchParams::default();

        let search = e::search_projects_iter(&CLIENT, &API_BASE, params);
        pin!(search);

        while let Some(result) = search.next().await {
            println!("{:?}", result);
        }
    })
}

// #[test]
// fn search_projects() {
//     smol::block_on(async {
//         // use smol::pin;
//         // use smol::stream::StreamExt;

//         let params = ProjectSearchParams::default();

//         let search = e::search_projects(&CLIENT, &API_BASE, &params).await;

//         dbg!(search);
//     })
// }
