use modrinth::prelude::*;

fn main() {
    let mut results = get_search_iter(
        SearchParams {
            query: None,
            facets: None,
            index: None,
            offset: None,
            limit: None,
            filters: None,
        },
        None,
    );

    println!(
        "result: {:?}\nsize_hint: {:?}",
        results.next().unwrap(),
        results.size_hint()
    );

    for result in &mut results {
        println!("result: {:?}", result);
    }

    println!("error: {:?}", results.error());
}
