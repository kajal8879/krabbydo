
use middleware::EventEntry;
//use notify_rust::Notification;


pub fn print_today_events()-> Result<Vec<EventEntry>, Box<dyn std::error::Error>>  {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let notification_task_list = rt.block_on(async {EventEntry::get_today_events().await.unwrap()});
    println!("{:?}", notification_task_list);
    Ok(notification_task_list)
}


fn main(){
    let notification_task_list=print_today_events().unwrap();
    for notification_task in notification_task_list {
        // Perform actions on each notification_task
        // For example, print the task name
        println!("Task Name: {}", notification_task.is_done);
    }

//let result = rt.block_on(async { EventEntry::get_today_events().await });
}

#[test]
fn test_print_today_events() {
    print_today_events().unwrap();
}
