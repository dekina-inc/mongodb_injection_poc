use mongodb::{bson::doc, options::DropCollectionOptions, Collection, Database};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct TestCollection {
    id: String,
    pw: String,
}

impl TestCollection {
    pub fn collection(database: Database) -> Collection<Self> {
        database.collection("test")
    }
}

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    log::info!("Start testing injection");

    log::info!("Connecting to MongoDB");
    let db_connection = mongodb::Client::with_uri_str("mongodb://localhost:27017/")
        .await
        .expect("Failed to connect to MongoDB");

    let db = db_connection.database("hacking_poc");

    let test_collection = TestCollection::collection(db.clone());

    // Clear existing data

    test_collection
        .drop(DropCollectionOptions::default())
        .await
        .unwrap();

    // Insert data

    let test_datas = [
        TestCollection {
            id: "test1".to_string(),
            pw: "test1".to_string(),
        },
        TestCollection {
            id: "test2".to_string(),
            pw: "test2".to_string(),
        },
        TestCollection {
            id: "test3".to_string(),
            pw: "test3".to_string(),
        },
    ];

    test_collection.insert_many(test_datas, None).await.unwrap();

    // Normal query

    let user_id_input = "test1";
    let user_pw_input = "test1";
    let query = doc! { "id": user_id_input, "pw": user_pw_input};

    let result = test_collection
        .find_one(query, None)
        .await
        .unwrap()
        .unwrap();

    log::info!("Normal query result: {:?}", result);

    // Injection query

    let user_id_input = r#"{"$ne": null}"#;
    let user_pw_input = r#"{"$ne": null}"#;
    let query = doc! { "id": user_id_input, "pw": user_pw_input };

    let result = test_collection.find_one(query, None).await.unwrap();

    log::info!("Injection query result: {:?}", result);

    // Injection query normal hard input test

    let query = doc! { "id": {
            "$ne": "null"
        }, "pw": {
            "$ne": "null"
        }
    };

    let result = test_collection.find_one(query, None).await.unwrap();

    log::info!(
        "Injection query normal hard input test result: {:?}",
        result
    );
}
