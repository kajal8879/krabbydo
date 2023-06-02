// notifications/src/lib.rs
#![warn(clippy::all, rust_2018_idioms)]

use chrono::{DateTime, Utc};
use middleware::EventEntry;

#[derive(Debug, Clone)]
pub struct Notification {
    id: String,
    event_id: String,
    message: String,
    datetime: DateTime<Utc>,
}

impl Notification {
    pub fn new(id: String, event_id: String, message: String, datetime: DateTime<Utc>) -> Self {
        Self {
            id,
            event_id,
            message,
            datetime,
        }
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    pub fn get_event_id(&self) -> &str {
        &self.event_id
    }

    pub fn get_datetime(&self) -> &DateTime<Utc> {
        &self.datetime
    }

    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }
}

pub struct NotificationManager {
    notifications: Vec<Notification>,
}

impl NotificationManager {
    pub fn new() -> Self {
        Self {
            notifications: vec![],
        }
    }

    pub fn add_notification(&mut self, notification: Notification) {
        self.notifications.push(notification);
    }

    pub fn remove_notification_by_id(&mut self, id: &str) {
        self.notifications.retain(|notif| notif.id != id);
    }

    pub fn get_notification_by_id(&self, id: &str) -> Option<&Notification> {
        self.notifications.iter().find(|notif| notif.id == id)
    }

    pub fn get_notifications_for_event(&self, event_id: &str) -> Vec<&Notification> {
        self.notifications
            .iter()
            .filter(|notif| notif.event_id == event_id)
            .collect()
    }

    pub fn update_notification(&mut self, id: &str, message: String) {
        if let Some(notif) = self.notifications.iter_mut().find(|notif| notif.id == id) {
            notif.set_message(message);
        }
    }

    pub fn list_notifications(&self) {
        for notif in &self.notifications {
            println!("Notification ID: {}", notif.id);
            println!("Event ID: {}", notif.event_id);
            println!("Message: {}", notif.message);
            println!("Time: {}", notif.datetime);
            println!("------------------------------");
        }
    }
}

#[test]
fn test_add_and_remove_notification() {
    let mut notif_manager = NotificationManager::new();
    let notif = Notification::new(
        "1".to_string(),
        "event1".to_string(),
        "Event 1 is starting soon.".to_string(),
        Utc::now(),
    );
    notif_manager.add_notification(notif);

    assert!(notif_manager.get_notification_by_id("1").is_some());
    notif_manager.remove_notification_by_id("1");
    assert!(notif_manager.get_notification_by_id("1").is_none());
}
