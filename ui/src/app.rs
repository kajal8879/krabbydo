use chrono::offset::*;
use chrono::DateTime;
use chrono::Datelike;
use chrono::NaiveDate;
use chrono::Timelike;
use egui::{
    menu, widgets, Align, CentralPanel, Checkbox, Direction, DragValue, Label, Layout, ScrollArea,
    SidePanel, TopBottomPanel, Ui, Window,
};
use middleware::EventEntry;
use bson::oid::ObjectId;
use std::fs::File;
use std::io::prelude::*;
use serde_json;

// https://stackoverflow.com/questions/48071513/how-to-use-one-module-from-another-module-in-a-rust-cargo-project
// GUI elements' dimension values segregated in a different file for ease of modification
#[path = "style.rs"]
mod style;
use style::style_constants;

// egui template sourced from:
// https://github.com/emilk/eframe_template

/// Enum to allow a user to specify AM or PM while choosing a time for an event
#[derive(PartialEq)]
enum AmPm {
    Am,
    Pm,
}

/// Struct to store UI components of Krabby Do
pub struct KrabbyDoUi {
    /// To control the display of New / Edit Event dialog
    is_show_new_edit_dialog: bool,

    /// To control the display of Edit Event button
    is_show_central_panel_context_elements: bool,

    /// To store the value of Title field in New / Edit Event dialog
    new_event_title: String,

    /// To store the value of Details field in New / Edit Event dialog
    new_event_details: String,

    /// To store the value of Date field in New / Edit Event dialog
    new_event_date: Option<NaiveDate>,

    /// To store the value of Hour field in New / Edit Event dialog
    new_event_hour: u32,

    /// To store the value of Minute field in New / Edit Event dialog
    new_event_minute: u32,

    /// To store the user's choice among AM and PM options in New / Edit Event dialog
    new_event_am_pm: AmPm,

    /// To specify if an event is done or not in New / Edit Event dialog
    new_event_is_done: bool,

    /// To store the value of date and time in a unified format
    date_time: DateTime<Utc>,

    /// Vector of event entries
    event_entries: Vec<EventEntry>,

    /// To hold the value for Title to be displayed in the event details panel
    details_panel_title: String,

    /// To hold the value for Details to be displayed in the event details panel
    details_panel_details: String,

    /// To hold the value for date and time to be displayed in the event details panel
    details_panel_time: String,

    /// To hold the currently selected event entry for editing purpose
    active_entry: EventEntry,

    /// To set title of the new/edit dialog as per current use
    new_edit_title: String,

    /// To search for a specific event based on various criteria
    search_query: String,

    new_event_tags: String,
}

impl Default for KrabbyDoUi {
    /// Assign default values to struct properties
    fn default() -> Self {
        Self {
            is_show_new_edit_dialog: false,
            is_show_central_panel_context_elements: false,
            new_event_title: "".to_owned(),
            new_event_details: "".to_owned(),
            new_event_is_done: false,
            new_event_date: None,
            new_event_hour: 6,
            new_event_minute: 30,
            new_event_am_pm: AmPm::Pm,
            search_query: String::new(),
            new_event_tags: String::new(),
            date_time: Utc.with_ymd_and_hms(2023, 5, 20, 22, 2, 0).unwrap(),
            event_entries: tokio::runtime::Runtime::new().unwrap().block_on(async { EventEntry::get_all_tasks().await }).unwrap(),
            details_panel_title: String::from("Krabby Do"),
            details_panel_details: String::from(""),
            details_panel_time: String::from(""),
            active_entry: EventEntry {
                unique_id: ObjectId::new(),
                title: String::from(""),
                details: String::from(""),
                date_time: Utc.with_ymd_and_hms(2000, 1, 1, 1, 1, 1).unwrap(),
                is_done: false,
                tags: String::new(),
            },
            new_edit_title: String::from("New Event"),
        }
    }
}

impl KrabbyDoUi {
    /// New function to set up the UI
    pub fn new() -> Self {
        Default::default()
    }

    /// Handle New Event menu option clicked;
    /// 1. Show New / Edit Event dialog
    /// 2. Load current time hours and minutes into their corresponding fields in dialog
    pub fn handle_menu_new_clicked(&mut self) {
        self.new_edit_title = String::from("New Event");
        self.is_show_new_edit_dialog = true;
        self.new_event_title = String::from("");
        self.new_event_details = String::from("");

        let now = Local::now();
        self.new_event_date = NaiveDate::from_ymd_opt(now.year(), now.month(), now.day());

        self.new_event_am_pm = AmPm::Am;
        let mut hour = Local::now().time().hour();
        if hour > 11 {
            self.new_event_am_pm = AmPm::Pm;
        }
        if hour > 12 {
            hour -= 12;
        } else if hour == 0 {
            hour = 12;
            self.new_event_am_pm = AmPm::Am;
        }
        self.new_event_hour = hour;
        let minute = Local::now().time().minute();
        self.new_event_minute = minute;

        self.new_event_is_done = false;
    }

    /// Handle OK button clicked of the New/Edit Event dialog;
    /// 1. Dump data of new event to log
    /// 2. Create a new event entry struct and populate it with the added data
    /// 3. Add the created struct to upcoming entries or marked-done entries as per user's choice
    pub fn handle_new_edit_ok_button_clicked(&mut self) {
        self.is_show_new_edit_dialog = false;
        let new_entry = EventEntry {
            unique_id: self.active_entry.unique_id,
            title: self.new_event_title.clone(),
            details: self.new_event_details.clone(),
            date_time: self.get_selected_date_time(),
            is_done: self.new_event_is_done,
            tags: self.new_event_tags.clone(),
        };

        #[cfg(feature = "print_debug_log")]
        println!("{:?}", new_entry);

        if self.new_edit_title == "New Event" {
            let _result = tokio::runtime::Runtime::new().unwrap().block_on(async { new_entry.add_event().await });
            self.event_entries.push(new_entry);
        } else if self.new_edit_title == "Edit Event" {
            #[cfg(feature = "print_debug_log")]
            println!("\nEntry edit requested!\n");
            
            let _result = tokio::runtime::Runtime::new().unwrap().block_on(async { new_entry.update_task().await });
            
            if let Some(index) = self
                .event_entries
                .iter()
                .position(|x| x == &(self.active_entry))
            {
                self.event_entries[index] = new_entry.clone();
                self.handle_event_list_item_clicked(&new_entry);
            }
        }
    }

    /// Handle Cancel button clicked of the New / Edit Event dialog; close the dialog
    pub fn handle_new_edit_cancel_button_clicked(&mut self) {
        self.is_show_new_edit_dialog = false;
    }

    /// Exports events to a JSON file.
    pub fn export_events_to_json(&self, filename: &str) -> std::io::Result<()> {
        // Serialize our events vector to a JSON string.
        let json = serde_json::to_string_pretty(&self.event_entries).unwrap();
        
        // Create a file and write the JSON data to it.
        let mut file = File::create(filename)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }

    /// Handle event list item clicked.; on clicking the event, display the event details in the central panel
    pub fn handle_event_list_item_clicked(&mut self, entry: &EventEntry) {
        // Currently clicked item is made ready to be loaded into Edit Dialog
        self.active_entry = entry.clone();

        // Edit Event button is made visible
        self.is_show_central_panel_context_elements = true;

        // Event data shown in the central panel
        self.details_panel_title = entry.title.clone();
        self.details_panel_details = entry.details.clone();

        // Event date time is displayed in the central panel
        // https://docs.rs/chrono/0.4.24/chrono/format/strftime/index.html
        self.details_panel_time = format!(
            "{}",
            entry
                .date_time
                .format("Date: %A, %B %e, %Y \tTime: %l:%M %p")
        );

        #[cfg(feature = "print_debug_log")]
        println!("{:?}", entry);
    }

    /// Handle Edit Event button clicked
    pub fn handle_edit_event_button_clicked(&mut self) {
        #[cfg(feature = "print_debug_log")]
        println!("\nEdit Event button clicked!\n");

        self.new_edit_title = String::from("Edit Event");
        self.is_show_new_edit_dialog = true;

        // Fetching string values to be displayed in Edit Dialog
        self.new_event_title = self.active_entry.title.clone();
        self.new_event_details = self.active_entry.details.clone();

        // Fetching date time of currently active event for editing
        let date_time = self.active_entry.date_time;

        // Fetching date values to be displayed in Edit Dialog
        self.new_event_date =
            NaiveDate::from_ymd_opt(date_time.year(), date_time.month(), date_time.day());

        // Fetching time values to be displayed in Edit Dialog
        if date_time.hour() > 11 {
            self.new_event_am_pm = AmPm::Pm;
        } else {
            self.new_event_am_pm = AmPm::Am;
        }
        if date_time.hour() > 12 {
            self.new_event_hour = date_time.hour() - 12;
        }
        self.new_event_minute = date_time.minute();
        self.new_event_is_done = self.active_entry.is_done;
    }

    /// Handle Edit button clicked on event list entry
    pub fn handle_event_list_item_edit_button_clicked(&mut self, entry: &EventEntry) {
        self.active_entry = entry.clone();
        self.handle_edit_event_button_clicked();
    }

    /// Handle Delete button clicked on event list entry
    pub fn handle_event_list_item_delete_button_clicked(&mut self, entry: &EventEntry) {
        let local_entry = &entry.clone();
        if let Some(index) = self.event_entries.iter().position(|x| x == local_entry) {
            self.event_entries.remove(index);
        }
    }

    /// Get the date selected by the date picker widget in NaiveDate format wrapped in Option
    pub fn get_selected_date(&mut self) -> Option<NaiveDate> {
        #[cfg(feature = "print_debug_log")]
        println!("Date selected: {}", self.new_event_date.unwrap_or_default());

        self.new_event_date
    }

    /// Get date and time selected by the user in the dialog in DateTime<Utc> format
    pub fn get_selected_date_time(&mut self) -> DateTime<Utc> {
        // Considering AM / PM
        let mut hour = self.new_event_hour;
        if hour < 12 && self.new_event_am_pm == AmPm::Pm {
            hour += 12;
        } else if hour == 12 && self.new_event_am_pm == AmPm::Am {
            hour -= 12;
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

    /// Function to create a UI list item for event list
    pub fn create_event_list_item(&mut self, ui: &mut Ui, entry: EventEntry) {
        ui.style_mut().spacing.item_spacing.y = style_constants::EVENT_LIST_BUTTON_SPACING;
        ui.with_layout(Layout::top_down(Align::TOP), |ui| {
            ui.set_min_width(50.0);
            ui.set_max_width(200.0);
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                ui.set_max_height(style_constants::EVENT_LIST_BUTTON_MAX_HEIGHT);
                ui.with_layout(
                    Layout::centered_and_justified(Direction::LeftToRight),
                    |ui| {
                        if ui.button(entry.title.clone()).clicked() {
                            KrabbyDoUi::handle_event_list_item_clicked(self, &entry);
                        }
                    },
                );
                ui.with_layout(
                    Layout::centered_and_justified(Direction::LeftToRight),
                    |ui| {
                        ui.set_min_width(40.0);
                        if ui.button("Edit").clicked() {
                            KrabbyDoUi::handle_event_list_item_edit_button_clicked(self, &entry);
                        }
                    },
                );
                ui.with_layout(
                    Layout::centered_and_justified(Direction::LeftToRight),
                    |ui| {
                        ui.set_min_width(40.0);
                        if ui.button("Delete").clicked() {
                            KrabbyDoUi::handle_event_list_item_delete_button_clicked(self, &entry);
                        }
                    },
                );
            });
        });
    }

    /// Generic function to populate a list of events using the event list supplied as an argument
    pub fn list_events(&mut self, ui: &mut Ui, widget_id: u32, is_show_events_marked_done: bool) {
        ui.push_id(widget_id, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                for entry in self.get_events() {
                    if is_show_events_marked_done {
                        if entry.is_done {
                            self.create_event_list_item(ui, entry);
                        }
                    } else {
                        if !entry.is_done {
                            self.create_event_list_item(ui, entry);
                        }
                    }
                }
            });
        });
    }

    /// Set up menu bar
    /// Set up menu bar
    pub fn setup_menu_bar(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("menu_panel").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Event").clicked() {
                        KrabbyDoUi::handle_menu_new_clicked(self);
                    }
                    if ui.button("Export").clicked() {
                        let filename = "exported_events.json";  // Filename can be dynamically determined.
                        match self.export_events_to_json(filename) {
                            Ok(_) => println!("Successfully exported events to {}", filename),
                            Err(e) => eprintln!("Error exporting events: {}", e),
                        }
                    }
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });

                ui.add(
                widgets::TextEdit::singleline(&mut self.search_query)
                        .hint_text("Search events"),
                );
            });
        });
    }

    /// Set up Left Panel that contains New Event button, Upcoming Events list and authors
    pub fn setup_left_panel(&mut self, ctx: &egui::Context) {
        SidePanel::left("left_side_panel").show(ctx, |ui| {
            ui.style_mut().spacing.item_spacing.y = style_constants::LEFT_PANEL_VERTICAL_SPACING;
            ui.with_layout(Layout::left_to_right(Align::TOP), |_ui| {});
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                    ui.set_max_height(style_constants::NEW_EVENT_BUTTON_MAX_HEIGHT);
                    ui.with_layout(
                        Layout::centered_and_justified(Direction::LeftToRight),
                        |ui| {
                            if ui.button("New Event").clicked() {
                                KrabbyDoUi::handle_menu_new_clicked(self);
                            }
                        },
                    );
                });
            });
            ui.separator();
            ui.heading("Upcoming Events");
            ui.with_layout(Layout::left_to_right(Align::TOP), |_ui| {});
            self.list_events(ui, 123456, false);
            ui.separator();
            ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Rohan, Kajal, Prachi");
                });
            });
        });
    }

    /// Set up Right Panel that contains list of events marked done
    pub fn setup_right_panel(&mut self, ctx: &egui::Context) {
        SidePanel::right("right_side_panel").show(ctx, |ui| {
            ui.style_mut().spacing.item_spacing.y = style_constants::RIGHT_PANEL_VERTICAL_SPACING;
            ui.with_layout(Layout::left_to_right(Align::TOP), |_ui| {});
            ui.heading("Marked Done");
            ui.with_layout(Layout::left_to_right(Align::TOP), |_ui| {});
            self.list_events(ui, 123457, true);
            ui.separator();
        });
    }

    /// Set up central panel to display event details of the currently selected event
    pub fn setup_central_panel(&mut self, ctx: &egui::Context) {
        CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().spacing.item_spacing.y = style_constants::CENTRAL_PANEL_VERTICAL_SPACING;
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                    ui.set_max_height(style_constants::CENTRAL_PANEL_TITLE_MAX_HEIGHT);
                    ui.with_layout(
                        Layout::centered_and_justified(Direction::LeftToRight),
                        |ui| {
                            ui.heading(self.details_panel_title.clone());
                        },
                    );
                });
            });
            if self.is_show_central_panel_context_elements {
                ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                    ui.set_min_width(style_constants::DETAILS_PANEL_MINIMUM_WIDTH);
                    ui.add(Label::new(self.details_panel_details.clone()).wrap(true));
                });
                ui.separator();
                ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                    ui.add(Label::new(self.details_panel_time.clone()).wrap(true));
                });
                ui.separator();
                ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                    ui.label("Tags:");
                    ui.label(self.active_entry.tags.clone());
                });
                ui.separator();
                ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                    ui.set_max_height(style_constants::EDIT_EVENT_BUTTON_MAX_HEIGHT);
                    ui.with_layout(
                        Layout::centered_and_justified(Direction::LeftToRight),
                        |ui| {
                            if ui.button("Edit Event").clicked() {
                                KrabbyDoUi::handle_edit_event_button_clicked(self);
                            }
                        },
                    );
                });
            }
        });
    }

    /// Set up New / Edit Event dialog
    pub fn setup_new_event_dialog(&mut self, ctx: &egui::Context) {
        self.new_event_hour = self.new_event_hour.clamp(1, 12);
        self.new_event_minute = self.new_event_minute.clamp(0, 60);

        Window::new(self.new_edit_title.clone()).show(ctx, |ui| {
            ui.style_mut().spacing.item_spacing.y =
                style_constants::NEW_EDIT_DIALOG_VERTICAL_SPACING;
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                    ui.set_min_width(style_constants::NEW_EDIT_DIALOG_MIN_LABEL_WIDTH);
                    ui.label("Title");
                });
                ui.add(
                    widgets::TextEdit::singleline(&mut self.new_event_title)
                        .hint_text("Enter event title"),
                );
            });
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                    ui.set_min_width(style_constants::NEW_EDIT_DIALOG_MIN_LABEL_WIDTH);
                    ui.label("Details");
                });
                ui.add(
                    widgets::TextEdit::multiline(&mut self.new_event_details)
                        .hint_text("Enter event details"),
                );
            });
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                    ui.set_min_width(style_constants::NEW_EDIT_DIALOG_MIN_LABEL_WIDTH);
                    ui.label("Date");
                });
                let date = self
                    .new_event_date
                    .get_or_insert_with(|| chrono::offset::Utc::now().date_naive());
                ui.add(egui_extras::DatePickerButton::new(date));
            });
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                    ui.set_min_width(style_constants::NEW_EDIT_DIALOG_MIN_LABEL_WIDTH);
                    ui.label("Time");
                });
                ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                        ui.set_min_width(style_constants::NEW_EDIT_DIALOG_MIN_LABEL_WIDTH);
                        ui.label("Hour");
                        ui.add(DragValue::new(&mut self.new_event_hour).speed(0.1));
                    });
                    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                        ui.set_min_width(style_constants::NEW_EDIT_DIALOG_MIN_LABEL_WIDTH);
                        ui.label("Minute");
                        ui.add(DragValue::new(&mut self.new_event_minute).speed(0.1));
                    });
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.new_event_am_pm, AmPm::Am, "AM");
                        ui.selectable_value(&mut self.new_event_am_pm, AmPm::Pm, "PM");
                    });
                });
            });
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                    ui.set_min_width(style_constants::NEW_EDIT_DIALOG_MIN_LABEL_WIDTH);
                    ui.label("Mark Done");
                });
                ui.add(Checkbox::new(&mut self.new_event_is_done, ""));
            });
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                    ui.set_min_width(style_constants::NEW_EDIT_DIALOG_MIN_LABEL_WIDTH);
                    ui.label("Tags");
                });
                ui.horizontal(|ui| {
                    ui.add(
                        widgets::TextEdit::multiline(&mut self.new_event_tags)
                            .hint_text("Enter tags"),
                    );
                });
            });
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                ui.set_max_width(style_constants::NEW_EDIT_DIALOG_MAX_WIDTH);
                ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
                    if ui.button("Cancel").clicked() {
                        KrabbyDoUi::handle_new_edit_cancel_button_clicked(self);
                    } else if ui.button("OK").clicked() {
                        KrabbyDoUi::handle_new_edit_ok_button_clicked(self);
                    }
                });
            });
        });
    }

    pub fn get_events(&self) -> Vec<EventEntry> {
        let search_query = self.search_query.to_lowercase(); // Convert search query to lowercase for case-insensitive search

        self.event_entries
            .iter()
            .filter(|event| {
                event.title.to_lowercase().contains(&search_query)
                    || event.details.to_lowercase().contains(&search_query)
                    || event
                        .tags.to_lowercase().contains(&search_query)
                        // .any(|tag| tag.to_lowercase().contains(&search_query))
            })
            .cloned()
            .collect()
    }
}

impl eframe::App for KrabbyDoUi {
    /// Update the state of UI (Redraw UI)
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self { .. } = self;

        // Menu Bar
        self.setup_menu_bar(ctx, frame);

        // Left Panel
        self.setup_left_panel(ctx);

        // Right Panel
        self.setup_right_panel(ctx);

        // Central Panel
        self.setup_central_panel(ctx);

        if self.is_show_new_edit_dialog {
            // New / Edit Event dialog
            self.setup_new_event_dialog(ctx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_selected_date() {
        let mut test_ui = KrabbyDoUi::default();
        test_ui.new_event_date = NaiveDate::from_ymd_opt(2023, 6, 9);
        let test_date = NaiveDate::from_ymd_opt(2023, 6, 9).unwrap();
        assert_eq!(test_ui.get_selected_date().unwrap(), test_date);
    }

    #[test]
    fn test_get_selected_date_time() {
        let mut test_ui = KrabbyDoUi::default();
        test_ui.new_event_date = NaiveDate::from_ymd_opt(2023, 6, 9);
        test_ui.new_event_hour = 15;
        test_ui.new_event_minute = 9;
        let test_date_time = Utc.with_ymd_and_hms(2023, 6, 9, 15, 9, 0).unwrap();
        assert_eq!(test_ui.get_selected_date_time(), test_date_time);
    }
}
