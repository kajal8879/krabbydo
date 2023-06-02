use chrono::{DateTime, Utc};
use mongodb::{Client, options::ClientOptions};
use mongodb::bson::{doc};
use tokio::runtime::Runtime;

/// Struct to store event data
#[derive(Debug, Clone)]
pub struct EventEntry {
    pub unique_id: String,
    pub title: String,
    pub details: String,
    pub date_time: DateTime<Utc>,
    pub is_done: bool,
}

impl EventEntry {
    pub fn new(unique_id: String, title: String, details: String, date_time: DateTime<Utc>, is_done: bool) -> Self {
        EventEntry {
            unique_id,
            title,
            details,
            date_time,
            is_done,
        }   
    }

    pub async fn add_event(&self, client: Client) -> Result<(), Box<dyn std::error::Error>> {
        println!("Event added to MongoDB");

        // Get a handle to the "todos" collection in the "tasks" database
        let db = client.database("events");
        let collection = db.collection("todos");

        // Create a document representing the ToDo task
        let document = doc! {
            "title": self.title.clone(),
            "details": self.details.clone(),
            "date_time": self.date_time.to_rfc3339(),
        };

        // Insert the document into the collection
        collection.insert_one(document, None).await?;

        Ok(())
    }

    pub fn update_task(&self) {
        println!("Event Updated !")
    }

    pub fn delete_or_mark_completed(&self) {
        println!("Event Completed !")
    }
}

pub async fn create_mongodb_client() -> Result<Client, Box<dyn std::error::Error>> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;
    Ok(client)
}

#[test]
fn test_add_task() {
    let task_name = String::from("KrabbyDo new setup");
    let task_desc = String::from("First input Done! Mongo Setup successful");
    let reminder_time = Utc::now();
    let is_completed = false;

    let event_entry = EventEntry::new(String::from(""), task_name, task_desc, reminder_time, is_completed);
    let rt = Runtime::new().unwrap();

     // Run the add_task function asynchronously
     let result = rt.block_on(async {
        let client = create_mongodb_client().await?;
        event_entry.add_event(client).await
    });

    // Assert that the add_task function succeeded
    assert!(result.is_ok(), "add_event failed");
}
