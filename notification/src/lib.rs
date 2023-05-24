pub trait Observer {
    fn on_notify(&self, event: &Event);
}

pub struct Subject {
    observers: Vec<Box<dyn Observer>>,
}

impl Subject {
    pub fn new() -> Self {
        Self {
            observers: Vec::new(),
        }
    }

    pub fn add_observer(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    pub fn remove_observer(&mut self, index: usize) {
        self.observers.remove(index);
    }

    pub fn notify_observers(&self, event: &Event) {
        for observer in &self.observers {
            observer.on_notify(event);
        }
    }
}

pub struct Event {
    pub message: String,
    pub file_name: String,
}

pub struct PrintObserver;

impl Observer for PrintObserver {
    fn on_notify(&self, event: &Event) {
        println!("Notified with message: {} from file: {}", event.message, event.file_name);
    }
}

fn main() {
    let mut subject = Subject::new();
    let print_observer = Box::new(PrintObserver);

    subject.add_observer(print_observer);
    subject.notify_observers(&Event {
        message: String::from("An event occurred"),
        file_name: String::from("example.txt"),
    });
}
