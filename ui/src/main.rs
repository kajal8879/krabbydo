#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

/// Main function to initiate the execution of Krabby Do
fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Krabby Do",
        native_options,
        Box::new(|_| Box::new(ui::KrabbyDoUi::new())),
    )
}
