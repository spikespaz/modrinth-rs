use std::fs::File;
use std::io::Write;

use smol::pin;
use smol::stream::StreamExt;

use modrinth::prelude::*;

fn main() {
    smol::block_on(async {
        let modrinth = Client::new(None);

        let params = SearchParams {
            query: None,
            facets: None,
            index: None,
            offset: None,
            limit: None,
            filters: None,
        };

        let search = modrinth.search_iter(params.clone());
        pin!(search);

        let mut output_file = File::create("projects_search.txt").unwrap();

        while let Some(project) = search.next().await {
            writeln!(output_file, "{:?}", project.unwrap()).unwrap();
        }
    });
}
