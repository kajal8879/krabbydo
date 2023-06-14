use std::str::FromStr;

use chrono::{DateTime, Utc};
use mongodb::bson::{doc, oid::ObjectId, Bson, Document};
use mongodb::Collection;
use mongodb::{options::ClientOptions, Client};
use tokio_stream::StreamExt as TokioStreamExt;

/// Struct to store event data
#[derive(Debug, Clone, PartialEq)]
pub struct EventEntry {
    pub unique_id: ObjectId,
    pub title: String,
    pub details: String,
    pub date_time: DateTime<Utc>,
    pub is_done: bool,
}

impl EventEntry {
    pub fn new(
        unique_id: ObjectId,
        title: String,
        details: String,
        date_time: DateTime<Utc>,
        is_done: bool,
    ) -> Self {
        EventEntry {
            unique_id,
            title,
            details,
            date_time,
            is_done,
        }
    }

    pub async fn add_event(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Event added to MongoDB");
        let client = create_mongodb_client().await?;
        // Get a handle to the "todos" collection in the "tasks" database
        let db = client.database("events");
        let collection = db.collection("todos");

        // Create a document representing the ToDo task
        let document = doc! {
            "title": self.title.clone(),
            "details": self.details.clone(),
            "date_time": self.date_time.to_rfc3339(),
            "is_done":false,
        };

        // Insert the document into the collection
        collection.insert_one(document, None).await?;

        Ok(())
    }
    pub async fn update_task(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Updating event with unique_id: {}", self.unique_id);
        let client = create_mongodb_client().await?;
        // Get a handle to the "todos" collection in the "tasks" database
        let db = client.database("events");
        let collection: Collection<Document> = db.collection("todos");

        // Create a document representing the ToDo task

        let filter = doc! { "_id":self.unique_id };
        let update = doc! { "$set": { "title": self.title.clone(), "details": self.details.clone(),"date_time": self.date_time.to_rfc3339(),
        "is_done": self.is_done, } };

        // Insert the document into the collection
        collection.update_one(filter, update, None).await?;

        Ok(())
    }

    pub async fn delete_event(&self) -> Result<(), Box<dyn std::error::Error>> {
        let client = create_mongodb_client().await?;
        let db = client.database("events");
        let collection: Collection<Document> = db.collection("todos");

        // Define the filter to find the event by its unique_id
        let filter = doc! { "_id": self.unique_id };

        // Update the document in the collection
        collection.delete_one(filter, None).await?;

        Ok(())
    }

    pub async fn get_all_tasks() -> Result<Vec<EventEntry>, Box<dyn std::error::Error>> {
        let client = create_mongodb_client().await?;
        let db = client.database("events");
        let collection: Collection<Document> = db.collection("todos");

        // Find all documents in the collection
        let mut cursor = collection.find(None, None).await?;

        // Iterate over the cursor using the `try_next` method
        let mut tasks = Vec::new();

        while let Some(result) = TokioStreamExt::try_next(&mut cursor).await? {
            // Extract task data from the document
            let unique_id = match result.get("_id") {
                Some(Bson::ObjectId(object_id)) => *object_id,
                _ => return Err("Invalid unique_id".into()), // Handle the error as you prefer
            };
            let title = result.get_str("title")?.to_string();
            let details = result.get_str("details")?.to_string();
            let date_time_str = result.get_str("date_time")?;
            let date_time = DateTime::parse_from_rfc3339(date_time_str)?.with_timezone(&Utc);
            let is_done = result.get_bool("is_done")?;

            // Create a new EventEntry instance
            let task = EventEntry::new(unique_id, title, details, date_time, is_done);

            // Add the task to the vector
            tasks.push(task);
        }
        println!("Tasks: {:?}", tasks);
        Ok(tasks)
    }
    pub async fn get_today_events() -> Result<Vec<EventEntry>, Box<dyn std::error::Error>> {
        let client = create_mongodb_client().await?;
        let db = client.database("events");
        let collection: Collection<Document> = db.collection("todos");

        let now = Utc::now();
        let today = now.date_naive().and_hms_opt(0, 0, 0).unwrap();
        let tomorrow = today + chrono::Duration::days(1);

        // Filter documents based on the date range from today to tomorrow
        let filter = doc! {
            "date_time": {
                "$gte": today.to_string(),
                "$lt": tomorrow.to_string()
            }
        };

        // Find documents that match the filter
        let mut cursor = collection.find(filter, None).await?;

        let mut tasks = Vec::new();

        while let Some(result) = TokioStreamExt::try_next(&mut cursor).await? {
            let unique_id = match result.get("_id") {
                Some(Bson::ObjectId(object_id)) => object_id.clone(),
                _ => return Err("Invalid unique_id".into()),
            };
            let title = result.get_str("title")?.to_string();
            let details = result.get_str("details")?.to_string();
            let date_time_str = result.get_str("date_time")?;
            let date_time = DateTime::parse_from_rfc3339(date_time_str)?.with_timezone(&Utc);
            let is_done = result.get_bool("is_done")?;

            let task = EventEntry::new(unique_id, title, details, date_time, is_done);
            tasks.push(task);
        }
        println!("Tasks: {:?}", tasks);
        Ok(tasks)
    }
}

pub async fn create_mongodb_client() -> Result<Client, Box<dyn std::error::Error>> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;
    Ok(client)
}

#[test]
fn test_update_task() {
    let task_name = String::from("KrabbyDo new setup 4");
    let task_desc = String::from("update input Done! Mongo Setup successful3");
    let reminder_time = Utc::now();
    let is_completed = false;
    let mongo_id = ObjectId::from_str("6482a04d44d9bc1cff4c66d7").unwrap();

    let event_entry = EventEntry::new(mongo_id, task_name, task_desc, reminder_time, is_completed);
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async { event_entry.update_task().await });

    // Assert that the add_task function succeeded
    assert!(result.is_ok(), "_event failed");
}
#[test]
fn test_add_task() {
    let task_name = String::from("KrabbyDo new setup 6");
    let task_desc = String::from("First input Done! Mongo Setup successful3");
    let reminder_time = Utc::now();
    let is_completed = false;

    let unique_id = ObjectId::new();
    let event_entry = EventEntry::new(unique_id, task_name, task_desc, reminder_time, is_completed);
    let rt = tokio::runtime::Runtime::new().unwrap();

    // Run the add_task function asynchronously
    let result = rt.block_on(async { event_entry.add_event().await });

    // Assert that the add_task function succeeded
    assert!(result.is_ok(), "add_event failed");
}
#[test]

fn test_get_all_tasks() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    // Run the get_all_tasks function asynchronously
    let result = rt.block_on(async { EventEntry::get_all_tasks().await });

    // Assert that the get_all_tasks function succeeded
    assert!(result.is_ok(), "get_all_tasks failed");
}
#[test]
fn test_delete_event() {
    let task_name = String::from("KrabbyDo new setup");
    let task_desc = String::from("Test delete_or_mark_completed");
    let reminder_time = Utc::now();
    let is_completed = false;
    let mongo_id = ObjectId::from_str("6482a04d44d9bc1cff4c66d7").unwrap();

    let event_entry = EventEntry::new(mongo_id, task_name, task_desc, reminder_time, is_completed);
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async { event_entry.delete_event().await });

    // Assert that the delete_or_mark_completed function succeeded
    assert!(result.is_ok(), "delete_event failed");
}
#[test]
fn test_get_today_events() {
    let rt = tokio::runtime::Runtime::new().unwrap();

    // Run the get_today_events function asynchronously
    let result = rt.block_on(async { EventEntry::get_today_events().await });

    // Assert that the get_today_events function succeeded
    assert!(result.is_ok(), "get_today_events failed");
}
