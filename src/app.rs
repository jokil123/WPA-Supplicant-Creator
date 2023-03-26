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
                match get_config_data() {
                    Ok(d) => self.config_data = d,
                    Err(e) => native_dialog::MessageDialog::new()
                        .set_type(native_dialog::MessageType::Error)
                        .set_title("Error")
                        .set_text(format!("Failed to Detect Informtation!\nError:\n{}", e).as_str())
                        .show_alert()
                        .unwrap(),
                };
            };

            egui::Grid::new("input").num_columns(2).show(ui, |ui| {
                let label_country = ui.label("Country");
                ui.text_edit_singleline(&mut self.config_data.country)
                    .labelled_by(label_country.id);
                ui.end_row();

                let label_ssid = ui.label("Wifi SSID");
                ui.text_edit_singleline(&mut self.config_data.network.ssid)
                    .labelled_by(label_ssid.id);
                ui.end_row();

                let label_password = ui.label("Wifi Password");
                ui.add(
                    egui::TextEdit::singleline(&mut self.config_data.network.password)
                        .password(true),
                )
                .labelled_by(label_password.id);
                ui.end_row();

                let label_output_path = ui.label("Output File Path");
                let mut path_string_buf = self.config_data.path.to_string_lossy().to_string();

                if ui
                    .text_edit_singleline(&mut path_string_buf)
                    .labelled_by(label_output_path.id)
                    .changed()
                {
                    self.config_data.path = PathBuf::from(path_string_buf);
                }
            });

            if ui.button("Create Config File").clicked() {
                match create_config_file(&self.config_data) {
                    Ok(_) => {
                        native_dialog::MessageDialog::new()
                            .set_type(native_dialog::MessageType::Info)
                            .set_title("Success")
                            .set_text("Config file created successfully!")
                            .show_alert()
                            .unwrap();
                    }
                    Err(e) => {
                        native_dialog::MessageDialog::new()
                            .set_type(native_dialog::MessageType::Error)
                            .set_title("Error")
                            .set_text(
                                format!("Config file creation failed!\nError:\n{}", e).as_str(),
                            )
                            .show_alert()
                            .unwrap();
                    }
                };
            };
        });
    }
}
