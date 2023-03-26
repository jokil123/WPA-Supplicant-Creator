use std::path::PathBuf;

use eframe::egui;

use crate::{
    create_config_file::create_config_file, data_types::ConfigData,
    get_config_data::get_config_data,
};

#[derive(Debug, Clone, Default)]
pub struct Ui {
    config_data: ConfigData,
}

impl eframe::App for Ui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("WPA Supplicant Creator");

            if ui.button("Detect").clicked() {
                self.config_data = get_config_data().unwrap();
            };

            egui::Grid::new("input").num_columns(2).show(ui, |ui| {
                let label = ui.label("Country");
                ui.text_edit_singleline(&mut self.config_data.country)
                    .labelled_by(label.id);
                ui.end_row();

                let label = ui.label("Wifi SSID");
                ui.text_edit_singleline(&mut self.config_data.network.ssid)
                    .labelled_by(label.id);
                ui.end_row();

                let label = ui.label("Wifi Password");
                ui.add(
                    egui::TextEdit::singleline(&mut self.config_data.network.password)
                        .password(true),
                );
                ui.end_row();

                let label = ui.label("Output File Path");
                let mut path_string_buf = self.config_data.path.to_string_lossy().to_string();

                if ui.text_edit_singleline(&mut path_string_buf).changed() {
                    self.config_data.path = PathBuf::from(path_string_buf);
                }
            });

            if ui.button("Create Config File").clicked() {
                create_config_file(&self.config_data).unwrap();
            };
        });
    }
}
