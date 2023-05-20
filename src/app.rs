/// egui template sourced from: 
/// https://github.com/emilk/eframe_template

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct KrabbyDo {
}

impl Default for KrabbyDo {
    fn default() -> Self {
        Self {
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

    pub fn handle_menu_new_clicked() {
        println!("\nNew Reminder menu option clicked!");
    }
    pub fn handle_new_ok_button_clicked() {
        println!("\nNew Reminder dialog OK button clicked!");
    }
    pub fn handle_new_cancel_button_clicked() {
        println!("\nNew Reminder dialog Cancel button clicked!");
    }
}

impl eframe::App for KrabbyDo {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {} = self;

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

        egui::Window::new("New Reminder").show(ctx, |ui| {
            ui.label("Title:\n");
            ui.label("Details:\n");
            ui.label("Date:\n");
            ui.label("Time:\n");
            if ui.button("OK").clicked() {
                KrabbyDo::handle_new_ok_button_clicked();
            } else if ui.button("Cancel").clicked() {
                KrabbyDo::handle_new_cancel_button_clicked();
            }
        });
    }
}
