//! This crate will generate the notifications for the event fetched from database which are due today
//!It uses notify-rust crate to do so.
use middleware::EventEntry;
use notify_rust::Notification;

///This function will call middleware crate to fetch events for due for today
fn fetch_events_for_today() -> Result<Vec<EventEntry>, Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let notification_task_list =
        rt.block_on(async { EventEntry::get_today_events().await.unwrap() });
    println!("{:?}", notification_task_list);
    Ok(notification_task_list)
}

///This function will generate notification for all events one by one
pub fn send_notifications() {
    let notification_task_list = fetch_events_for_today().unwrap();
    for notification_task in notification_task_list {
        // Perform actions on each notification_task
        if !notification_task.is_done {
            Notification::new()
                .summary(&notification_task.title)
                .body(&notification_task.details)
                .show()
                .unwrap();
        }
    }
}
