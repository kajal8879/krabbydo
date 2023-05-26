use chrono::offset::*;
use chrono::DateTime;
use chrono::Datelike;
use chrono::NaiveDate;
use chrono::Timelike;
use egui::{ScrollArea, Ui};

// egui template sourced from:
// https://github.com/emilk/eframe_template

/// Struct to store event data
pub struct EventEntry {
    title: String,
    details: String,
    date_time: DateTime<Utc>,
    is_done: bool,
}

/// Implementation of Clone trait for EventEntry struct
impl Clone for EventEntry {
    // https://levelup.gitconnected.com/rust-cloning-structs-explained-d633713d5de0
    /// Clone function implementation for EventEntry struct
    fn clone(&self) -> Self {
        EventEntry {
            title: self.title.clone(),
            details: self.details.clone(),
            date_time: self.date_time,
            is_done: self.is_done,
        }
    }
}

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
/// Enum to allow a user to specify AM or PM while choosing a time for an event
enum AmPm {
    Am,
    Pm,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
/// Struct to store UI components of Krabby Do
pub struct KrabbyDoUi {
    /// To control the display of New Event dialog
    #[serde(skip)]
    is_show_new_reminder_dialog: bool,

    /// To store the value of Title field in New Event dialog
    #[serde(skip)]
    new_event_title: String,

    /// To store the value of Details field in New Event dialog
    #[serde(skip)]
    new_event_details: String,

    /// To store the value of Title field in New Event dialog
    #[serde(skip)]
    new_event_date: Option<NaiveDate>,

    /// To store the value of Hour field in New Event dialog
    #[serde(skip)]
    new_event_hour: u32,

    /// To store the value of Minute field in New Event dialog
    #[serde(skip)]
    new_event_minute: u32,

    /// To store the user's choice among AM and PM options in New Event dialog
    new_event_am_pm: AmPm,

    /// To specify if an event is done or not in New Event dialog
    #[serde(skip)]
    new_event_is_done: bool,

    /// To store the value of date and time in a unified format
    #[serde(skip)]
    date_time: DateTime<Utc>,

    /// Vector of test entries
    #[serde(skip)]
    test_entries: Vec<EventEntry>,

    /// Vector of test entries marked done
    #[serde(skip)]
    test_entries_completed: Vec<EventEntry>,

    /// To hold the value for Title to be displayed in the event details panel
    #[serde(skip)]
    details_panel_title: String,

    /// To hold the value for Details to be displayed in the event details panel
    #[serde(skip)]
    details_panel_details: String,

    /// To hold the value for date and time to be displayed in the event details panel
    #[serde(skip)]
    details_panel_time: String,
}

impl Default for KrabbyDoUi {
    /// Assign default values to struct properties
    fn default() -> Self {
        Self {
            is_show_new_reminder_dialog: false,
            new_event_title: "".to_owned(),
            new_event_details: "".to_owned(),
            new_event_is_done: false,
            new_event_date: None,
            new_event_hour: 6,
            new_event_minute: 30,
            new_event_am_pm: AmPm::Pm,
            date_time: Utc.with_ymd_and_hms(2023, 5, 20, 22, 2, 0).unwrap(),
            test_entries: // https://doc.rust-lang.org/std/vec/struct.Vec.html
                vec![
                    EventEntry {
                    title: String::from("Alpha"),
                    details: String::from("Alpha - Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum"),
                    date_time: Utc.with_ymd_and_hms(2023, 5, 14, 22, 2, 0).unwrap(),
                    is_done: false,
                },
                EventEntry {
                    title: String::from("Bravo"),
                    details: String::from("Bravo - Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum"),
                    date_time: Utc.with_ymd_and_hms(2022, 5, 14, 22, 2, 0).unwrap(),
                    is_done: false,
                },
                EventEntry {
                    title: String::from("Charlie"),
                    details: String::from("Charlie - Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum"),
                    date_time: Utc.with_ymd_and_hms(2021, 5, 14, 22, 2, 0).unwrap(),
                    is_done: false,
                },
                EventEntry {
                    title: String::from("Delta"),
                    details: String::from("Delta - Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum"),
                    date_time: Utc.with_ymd_and_hms(2020, 5, 14, 22, 2, 0).unwrap(),
                    is_done: false,
                },
            ],
            test_entries_completed: vec![
                EventEntry {
                title: String::from("Echo"),
                details: String::from("Echo details"),
                date_time: Utc.with_ymd_and_hms(2019, 5, 14, 22, 2, 0).unwrap(),
                is_done: true,
                },
                EventEntry {
                    title: String::from("Foxtrot"),
                    details: String::from("Foxtrot details"),
                    date_time: Utc.with_ymd_and_hms(2018, 5, 14, 22, 2, 0).unwrap(),
                    is_done: true,
                },
                EventEntry {
                    title: String::from("Golf"),
                    details: String::from("Golf details"),
                    date_time: Utc.with_ymd_and_hms(2017, 5, 14, 22, 2, 0).unwrap(),
                    is_done: true,
                },
            ],
            details_panel_title: String::from("Krabby Do"),
            details_panel_details: String::from(""),
            details_panel_time: String::from(""),
        }
    }
}

impl KrabbyDoUi {
    /// New function to set up the UI
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }

    /// Handle New Event menu option clicked;
    /// 1. Show New Event dialog
    /// 2. Load current time hours and minutes into their corresponding fields in dialog
    pub fn handle_menu_new_clicked(&mut self) {
        self.is_show_new_reminder_dialog = true;
        self.new_event_am_pm = AmPm::Am;
        let mut hour = Local::now().time().hour();
        if hour > 11 {
            self.new_event_am_pm = AmPm::Pm;
        }
        if hour > 12 {
            hour -= 12;
        }
        self.new_event_hour = hour;
        let minute = Local::now().time().minute();
        self.new_event_minute = minute;
    }

    /// Handle OK button clicked of the New Event dialog;
    /// 1. Dump data of new event to log
    /// 2. Create a new event entry struct and populate it with the added data
    /// 3. Add the created struct to upcoming entries or marked-done entries as per user's choice
    pub fn handle_new_ok_button_clicked(&mut self) {
        println!("Event Title: {}", self.new_event_title);
        println!("Event Details: {}", self.new_event_details);
        self.is_show_new_reminder_dialog = false;
        println!("Date Time: {}", self.get_selected_date_time());
        println!("Is Done: {}", self.new_event_is_done);
        let new_entry = EventEntry {
            title: self.new_event_title.clone(),
            details: self.new_event_details.clone(),
            date_time: self.get_selected_date_time(),
            is_done: self.new_event_is_done,
        };
        if self.new_event_is_done {
            self.test_entries_completed.push(new_entry);
        } else {
            self.test_entries.push(new_entry);
        }
    }

    /// Handle Cancel button clicked of the New Event dialog; close the dialog
    pub fn handle_new_cancel_button_clicked(&mut self) {
        self.is_show_new_reminder_dialog = false;
    }

    /// Handle event list item clicked.; on clicking the event, display the event details in the central panel
    pub fn handle_event_list_item_clicked(&mut self, entry: &EventEntry) {
        println!("Event Title: {}", entry.title);
        println!("Event Details: {}", entry.details);
        println!("Event Date and Time: {}", entry.date_time);
        println!("Is Done: {}", entry.is_done);

        self.details_panel_title = entry.title.clone();
        self.details_panel_details = entry.details.clone();

        // https://docs.rs/chrono/0.4.24/chrono/format/strftime/index.html
        self.details_panel_time = format!(
            "{}",
            entry
                .date_time
                .format("Date: %A, %B %e, %Y \tTime: %l:%M %p")
        );
    }

    /// Get the date selected by the date picker widget in NaiveDate format wrapped in Option
    pub fn get_selected_date(&mut self) -> Option<NaiveDate> {
        println!("Date selected: {}", self.new_event_date.unwrap_or_default());
        self.new_event_date
    }

    /// Get date and time selected by the user in the dialog in DateTime<Utc> format
    pub fn get_selected_date_time(&mut self) -> DateTime<Utc> {
        // Considering AM / PM
        let mut hour = self.new_event_hour;
        if hour < 12 && self.new_event_am_pm == AmPm::Pm {
            hour += 12;
        }
        self.date_time = chrono::offset::Utc
            .with_ymd_and_hms(
                self.new_event_date.unwrap().year(),
                self.new_event_date.unwrap().month(),
                self.new_event_date.unwrap().day(),
                hour,
                self.new_event_minute,
                0,
            )
            .unwrap();
        self.date_time
    }

    /// Generic function to populate a list of events using the event list supplied as an argument
    pub fn list_events(&mut self, ui: &mut Ui, widget_id: u32, entries: Vec<EventEntry>) {
        ui.push_id(widget_id, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                for entry in entries {
                    ui.set_min_width(140.0);
                    ui.style_mut().spacing.item_spacing.y = 30.0;
                    ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
                        ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
                            if ui.button(entry.title.clone()).clicked() {
                                KrabbyDoUi::handle_event_list_item_clicked(self, &entry);
                            }
                        });
                    });
                }
            });
        });
    }

    /// Set up menu bar
    pub fn setup_menu_bar(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Event").clicked() {
                        KrabbyDoUi::handle_menu_new_clicked(self);
                    }
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });
    }

    /// Set up Left Panel that contains New Event button, Upcoming Events list and authors
    pub fn setup_left_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("left_side_panel").show(ctx, |ui| {
            ui.style_mut().spacing.item_spacing.y = 10.0;
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.set_min_width(10.0);
                });
            });

            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    if ui.button("New Event").clicked() {
                        KrabbyDoUi::handle_menu_new_clicked(self);
                    }
                });
            });
            ui.separator();
            ui.heading("Upcoming Events");
            ui.style_mut().spacing.item_spacing.y = 5.0;
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.set_min_width(10.0);
                });
            });
            self.list_events(ui, 123456, self.test_entries.clone());
            ui.separator();
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Rohan, Kajal, Prachi");
                });
            });
        });
    }

    /// Set up Right Panel that contains list of events marked done
    pub fn setup_right_panel(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("right_side_panel").show(ctx, |ui| {
            ui.style_mut().spacing.item_spacing.y = 10.0;
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.set_min_width(10.0);
                });
            });
            ui.heading("Marked Done");
            ui.style_mut().spacing.item_spacing.y = 5.0;
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.set_min_width(10.0);
                });
            });
            self.list_events(ui, 123457, self.test_entries_completed.clone());
            ui.separator();
        });
    }

    /// Set up central panel to display event details of the currently selected event
    pub fn setup_central_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().spacing.item_spacing.y = 30.0;
            ui.heading(self.details_panel_title.clone());
            ui.separator();
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.set_min_width(200.0);
                ui.add(egui::Label::new(self.details_panel_details.clone()).wrap(true));
            });
            ui.separator();
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.add(egui::Label::new(self.details_panel_time.clone()).wrap(true));
            });
            ui.separator();
        });
    }

    /// Set up New Event dialog
    pub fn setup_new_event_dialog(&mut self, ctx: &egui::Context) {
        const LABEL_WIDTH: f32 = 50.0;
        const Y_SPACING: f32 = 10.0;

        self.new_event_hour = self.new_event_hour.clamp(1, 12);
        self.new_event_minute = self.new_event_minute.clamp(0, 60);

        egui::Window::new("New Event").show(ctx, |ui| {
            ui.style_mut().spacing.item_spacing.y = Y_SPACING;
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.set_min_width(LABEL_WIDTH);
                    ui.label("Title");
                });
                ui.add(
                    egui::widgets::TextEdit::singleline(&mut self.new_event_title)
                        .hint_text("Enter event title"),
                );
            });
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.set_min_width(LABEL_WIDTH);
                    ui.label("Details");
                });
                ui.add(
                    egui::widgets::TextEdit::multiline(&mut self.new_event_details)
                        .hint_text("Enter event details"),
                );
            });
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.set_min_width(LABEL_WIDTH);
                    ui.label("Date");
                });
                let date = self
                    .new_event_date
                    .get_or_insert_with(|| chrono::offset::Utc::now().date_naive());
                ui.add(egui_extras::DatePickerButton::new(date));
            });
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.set_min_width(LABEL_WIDTH);
                    ui.label("Time");
                });
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                        ui.set_min_width(LABEL_WIDTH);
                        ui.label("Hour");
                        ui.add(egui::DragValue::new(&mut self.new_event_hour).speed(0.1));
                    });
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                        ui.set_min_width(LABEL_WIDTH);
                        ui.label("Minute");
                        ui.add(egui::DragValue::new(&mut self.new_event_minute).speed(0.1));
                    });
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.new_event_am_pm, AmPm::Am, "AM");
                        ui.selectable_value(&mut self.new_event_am_pm, AmPm::Pm, "PM");
                    });
                });
            });
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.set_min_width(LABEL_WIDTH);
                    ui.add(egui::Checkbox::new(
                        &mut self.new_event_is_done,
                        "Mark Done",
                    ));
                });
            });
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.set_max_width(330.0);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    if ui.button("Cancel").clicked() {
                        KrabbyDoUi::handle_new_cancel_button_clicked(self);
                    } else if ui.button("OK").clicked() {
                        KrabbyDoUi::handle_new_ok_button_clicked(self);
                    }
                });
            });
        });
    }
}

impl eframe::App for KrabbyDoUi {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { .. } = self;

        // Menu bar
        self.setup_menu_bar(ctx, _frame);

        // Left panel
        self.setup_left_panel(ctx);

        // Right Panel
        self.setup_right_panel(ctx);

        // Central Panel
        self.setup_central_panel(ctx);

        if self.is_show_new_reminder_dialog {
            // New Event dialog
            self.setup_new_event_dialog(ctx);
        }
    }
}
