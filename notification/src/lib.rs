
use middleware::EventEntry;
use notify_rust::Notification;

fn fetch_events_for_today()-> Result<Vec<EventEntry>, Box<dyn std::error::Error>>  {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let notification_task_list = rt.block_on(async {EventEntry::get_today_events().await.unwrap()});
    println!("{:?}", notification_task_list);
    Ok(notification_task_list)
}


pub fn send_notification(){
    let notification_task_list=fetch_events_for_today().unwrap();
    for notification_task in notification_task_list {
        // Perform actions on each notification_task
        if !notification_task.is_done {
            Notification::new()
            .summary(&notification_task.title)
            .body(&notification_task.details)
            .show().unwrap();
        }
    }

}

