use chrono::{DateTime, Utc};
use mongodb::{Client, options::ClientOptions};
use mongodb::bson::{doc};
use tokio::runtime::Runtime;

#[derive(Debug, Clone)]
pub struct ToDo {
    pub task_name: String,
    pub task_desc: String,
    pub reminder_time: DateTime<Utc>,
}

impl ToDo {
    pub fn new(task_name: String, task_desc: String, reminder_time: DateTime<Utc>) -> Self {
        ToDo {
            task_name,
            task_desc,
            reminder_time,
        }
    }

    pub async fn add_task(&self, client: Client) -> Result<(), Box<dyn std::error::Error>> {
        println!("task added to MongoDB");

        // Get a handle to the "todos" collection in the "tasks" database
        let db = client.database("tasks");
        let collection = db.collection("todos");

        // Create a document representing the ToDo task
        let document = doc! {
            "task_name": self.task_name.clone(),
            "task_desc": self.task_desc.clone(),
            "reminder_time": self.reminder_time.to_rfc3339(),
        };

        // Insert the document into the collection
        collection.insert_one(document, None).await?;

        Ok(())
    }

    pub fn update_task(&self) {
        println!("task updated !")
    }

    pub fn delete_or_mark_completed(&self) {
        println!("task Completed !")
    }
}

async fn create_mongodb_client() -> Result<Client, Box<dyn std::error::Error>> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;
    Ok(client)
}

#[test]
fn test_add_task() {
    let task_name = String::from("KrabbyDonew setup");
    let task_desc = String::from("First input Done! Mongo Setup successful");
    let reminder_time = Utc::now();

    let todo = ToDo::new(task_name, task_desc, reminder_time);
    let rt = Runtime::new().unwrap();

     // Run the add_task function asynchronously
     let result = rt.block_on(async {
        let client = create_mongodb_client().await?;
        todo.add_task(client).await
    });

    // Assert that the add_task function succeeded
    assert!(result.is_ok(), "add_task failed");
}
