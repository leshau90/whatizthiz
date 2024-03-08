// extern crate mongodb;
use mongodb::bson::doc;
use mongodb::{Client,options::ClientOptions};
#[tokio::main]
async fn main() {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client  =  Client::with_options(client_options).unwrap();

    let db_name = "rust-db";
    let coll_name = "test";

    create_collection(&client, db_name, coll_name).await;

    println!("created the document with db_name {} and coll_name {}",db_name,coll_name);
}

async fn create_collection(client: &Client, db_name:&str, coll_name:&str)-> Result<String,Error>{
    let db = client.database(db_name);
    db.create_collection(coll_name,None).await.unwrap()?;
    Ok("database")
}

async fn insert_document(client: &Client, db_name: &str,coll_name: &str){
    let db = client.database(db_name);
    let coll = db.collection(coll_name);
    let doc = doc!{"name":"ilman","job":"enl agent"};

    coll.insert_one(doc, None).await.unwrap();
}