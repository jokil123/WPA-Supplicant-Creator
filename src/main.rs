#![windows_subsystem = "windows"]
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] //Hide console window in release builds on Windows, this blocks stdout.

use eframe::egui;
use wpa_supplicant_creator::app::Ui;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 180.0)),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "WPA Supplicant Creator",
        options,
        Box::new(|_cc| Box::new(Ui::default())),
    )
    .unwrap();
}
