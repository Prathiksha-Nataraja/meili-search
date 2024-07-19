use meilisearch_sdk::{
    client::*,
    indexes::*,
    search::*,
    settings::*,
    tasks::{TasksPaginationFilters, TasksQuery, TasksSearchQuery},
};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read};
use tokio;

#[derive(Debug, Serialize, Deserialize)]
struct Movie {
    id: i64,
    title: String,
    poster: String,
    overview: String,
    release_date: i64,
    genres: Vec<String>,
}

#[tokio::main]
async fn main() {
    // block_on(async move {
    let client = Client::new("http://localhost:7700", Some("aSampleMasterKey")).unwrap();

    let mut file = File::open("/Users/prathiksha/Downloads/movies.json").unwrap();

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let movies_docs: Vec<Movie> = serde_json::from_str(&content).unwrap();

    //adding the documents
    let a = client
        .index("movies")
        .add_documents(&movies_docs, None)
        .await
        .unwrap();

    //use the return task Id to return the status [monitoring the task status]
    client.get_task(a.clone()).await.unwrap();

    //we can start searching here
    //we can search in the index directly
    let results: SearchResults<Movie> = client
        .index("movies")
        .search()
        .with_query("botman")
        // .with_limit(2)
        .execute()
        .await
        .unwrap();

    // println!("{:#?}", results);

    //Filtering the tasks with a single parameter
    let mut query = TasksSearchQuery::new(&client);
    // let tasks = query.with_statuses(["failed", "canceled"]).execute().await.unwrap();
    let tasks = query
        .with_index_uids(["movies"])
        .with_types(["documentAdditionOrUpdate", "documentDeletion"])
        .with_statuses(["processing"])
        .execute()
        .await
        .unwrap();
    println!("{:#?}", tasks);

    // })
}
