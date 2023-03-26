use eframe::{egui, AppCreator};
use wpa_supplicant_creator::{
    app::Ui, create_config_file::create_config_file, get_config_data::get_config_data,
};

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(320.0, 180.0)),
        ..Default::default()
    };

    eframe::run_native(
        "WPA Supplicant Creator",
        options,
        Box::new(|_cc| Box::new(Ui::default())),
    )
    .unwrap();
}
