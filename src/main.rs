#![feature(async_closure)]

use warp::Filter;
use serde_derive::{Deserialize, Serialize};

mod scrape;
use scrape::Scraper;

#[derive(Deserialize, Debug)]
struct Workload {
    selectors: Vec<String>,
    urls: Vec<String>,
    attrs: Vec<String>,
}

#[derive(Serialize, Debug)]
struct Response {
    count: usize,
}

async fn scrape(workload: Workload) -> Result<impl warp::Reply, warp::Rejection> {
    println!("workload {:#?}", workload);
    let scraper = Scraper::new(workload.selectors, workload.urls, workload.attrs);

    let response = scraper.elements().await.unwrap();

    Ok(warp::reply::json(&response))
}

#[tokio::main]
async fn main() {
    let scrape_route = warp::post()
        .and(warp::path("scrape"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(scrape);

    println!("Up and running, biatch");

    warp::serve(scrape_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
