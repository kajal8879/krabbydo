/// egui template sourced from: 
/// https://github.com/emilk/eframe_template

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct KrabbyDo {
    #[serde(skip)]
    is_show_new_reminder_dialog: bool,

    #[serde(skip)]
    is_date_picker_open: bool,

    #[serde(skip)]
    new_event_title: String,

    #[serde(skip)]
    new_event_details: String,

    #[serde(skip)]
    date: Option<chrono::NaiveDate>,
}

impl Default for KrabbyDo {
    fn default() -> Self {
        Self {
            is_show_new_reminder_dialog:true,
            is_date_picker_open: false,
            new_event_title: "".to_owned(),
            new_event_details: "".to_owned(),
            date: None,
        }
    }
}

impl KrabbyDo {
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
        self.handle_date_selected();
    }
    pub fn handle_new_cancel_button_clicked(&mut self) {
        self.is_show_new_reminder_dialog = false;
    }
    pub fn handle_date_selected(&mut self) {
        println!("Date selected: {}", self.date.unwrap_or_default());
    }
}

impl eframe::App for KrabbyDo {
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
                        KrabbyDo::handle_menu_new_clicked();
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
            egui::Window::new("New Reminder").show(ctx, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.label("Title");
                    ui.add(egui::widgets::TextEdit::singleline(&mut self.new_event_title).hint_text("Enter event title"));
                });
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.label("Details");
                    ui.add(egui::widgets::TextEdit::multiline(&mut self.new_event_details).hint_text("Enter event details"));
                });
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.label("Date\n");
                    let date = self.date.get_or_insert_with(|| chrono::offset::Utc::now().date_naive());
                    ui.add(egui_extras::DatePickerButton::new(date));
                });
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    ui.label("Time:\n");
                });
                ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                    if ui.button("OK").clicked() {
                        KrabbyDo::handle_new_ok_button_clicked(self);
                    } else if ui.button("Cancel").clicked() {
                        KrabbyDo::handle_new_cancel_button_clicked(self);
                    }
                });
            });
        }
    }
}
