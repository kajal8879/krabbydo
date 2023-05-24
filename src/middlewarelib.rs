use chrono::{DateTime, Utc};
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
    pub fn add_task(&self) {
        println!("task added to mongoDB")
    }
    pub fn update_task(&self) {
        println!("task updated !")
    }
    pub fn delete_or_mark_completed(&self) {
        println!("task Completed !")
    }
}

#[test]
fn test_add_task() {
    let task_name = String::from("Complete project");
    let task_desc = String::from("Finish the remaining tasks and submit the project");
    let reminder_time = Utc::now();

    let todo = ToDo::new(task_name, task_desc, reminder_time);
    todo.add_task();
}
