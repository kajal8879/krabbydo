#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Krabby Do",
        native_options,
        Box::new(|cc| Box::new(krabby_do::KrabbyDo::new(cc))),
    )
}
