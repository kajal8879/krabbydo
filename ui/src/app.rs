use chrono::offset::*;
use chrono::DateTime;
use chrono::Datelike;
use chrono::NaiveDate;

/// egui template sourced from:
/// https://github.com/emilk/eframe_template

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
enum AmPm {
    Am,
    Pm,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct KrabbyDoUi {
    #[serde(skip)]
    is_show_new_reminder_dialog: bool,

    #[serde(skip)]
    is_date_picker_open: bool,

    #[serde(skip)]
    new_event_title: String,

    #[serde(skip)]
    new_event_details: String,

    #[serde(skip)]
    date: Option<NaiveDate>,

    #[serde(skip)]
    hour: u32,

    #[serde(skip)]
    minute: u32,

    am_pm: AmPm,

    #[serde(skip)]
    date_time: DateTime<Utc>,
}

impl Default for KrabbyDoUi {
    fn default() -> Self {
        Self {
            is_show_new_reminder_dialog: true,
            is_date_picker_open: false,
            new_event_title: "".to_owned(),
            new_event_details: "".to_owned(),
            date: None,
            hour: 6,
            minute: 30,
            am_pm: AmPm::Pm,
            date_time: Utc.with_ymd_and_hms(2023, 5, 20, 22, 02, 0).unwrap(),
        }
    }
}

impl KrabbyDoUi {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }

    // You'll find the UI element action handler functions here.
    // I have printed out the values of variables involved in the UI for your
    // convenience, you can use these to interact with and implement the backend

    pub fn handle_menu_new_clicked() {
        println!("\nNew Reminder menu option clicked!");
    }
    pub fn handle_new_ok_button_clicked(&mut self) {
        println!("Event Title: {}", self.new_event_title);
        println!("Event Details: {}", self.new_event_details);
        self.is_show_new_reminder_dialog = false;
        self.get_selected_date();
        self.get_selected_date_time();
    }
    pub fn handle_new_cancel_button_clicked(&mut self) {
        self.is_show_new_reminder_dialog = false;
    }
    pub fn get_selected_date(&mut self) -> Option<NaiveDate> {
        println!("Date selected: {}", self.date.unwrap_or_default());
        self.date.clone()
    }
    pub fn get_selected_date_time(&mut self) -> DateTime<Utc> {
        // Considering AM / PM
        let mut hour = self.hour;
        if self.am_pm == AmPm::Pm {
            hour += 12;
        }
        self.date_time = chrono::offset::Utc
            .with_ymd_and_hms(
                self.date.unwrap().year(),
                self.date.unwrap().month(),
                self.date.unwrap().day(),
                hour,
                self.minute,
                0,
            )
            .unwrap();
        println!("Date Time: {}", self.date_time);
        self.date_time.clone()
    }
}

impl eframe::App for KrabbyDoUi {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // let Self {is_show_new_reminder_dialog} = self;
        egui::TopBottomPanel::top("menu_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Reminder").clicked() {
                        KrabbyDoUi::handle_menu_new_clicked();
                    }
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Upcoming Events");

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Rohan, Kajal, Prachi");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Krabby Do");
        });

        if self.is_show_new_reminder_dialog {
            const LABEL_WIDTH: f32 = 50.0;
            const Y_SPACING: f32 = 10.0;

            self.hour = self.hour.clamp(1, 12);
            self.minute = self.minute.clamp(0, 60);

            egui::Window::new("New Reminder").show(ctx, |ui| {
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
                        .date
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
                            ui.add(egui::DragValue::new(&mut self.hour).speed(0.1));
                        });
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                            ui.set_min_width(LABEL_WIDTH);
                            ui.label("Minute");
                            ui.add(egui::DragValue::new(&mut self.minute).speed(0.1));
                        });
                        ui.horizontal(|ui| {
                            ui.selectable_value(&mut self.am_pm, AmPm::Am, "AM");
                            ui.selectable_value(&mut self.am_pm, AmPm::Pm, "PM");
                        });
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
}
