use futures::executor::block_on;
use log::warn;

use crate::{model::FinItem, TemplateApp};

pub(crate) fn show(ui: &mut egui::Ui, _frame: &mut eframe::Frame, app: &mut TemplateApp) {
    egui::menu::bar(ui, |ui| {
        // menu bar starting from left
        ui.menu_button("File", |ui| {
            // Import button
            if ui.button("Import Data").clicked() {
                let future = async {
                    let file_option = rfd::AsyncFileDialog::new()
                        .add_filter("csv", &["csv"])
                        .set_directory("/")
                        .pick_file()
                        .await;

                    let data = file_option.unwrap().read().await;
                    let reader = std::io::BufReader::new(data.as_slice());
                    let mut rdr = csv::Reader::from_reader(reader);
                    for (i, result) in rdr.deserialize().enumerate() {
                        let record: Result<FinItem, csv::Error> = result;
                        match record {
                            Ok(item) => {
                                app.items.push(item);
                            }
                            Err(_) => {
                                warn!("Failed to parse line {i}");
                            }
                        }
                    }
                };

                block_on(future);
            }

            // Export button
            #[cfg(not(target_arch = "wasm32"))] // no File->Export on web pages!
            if ui.button("Export Data").clicked() {
                // TODO
            }

            // Quit button
            #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
            if ui.button("Quit").clicked() {
                _frame.close();
            }
        });

        // theme button on right
        ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
            egui::widgets::global_dark_light_mode_switch(ui);
            ui.label("Theme: ");
            egui::warn_if_debug_build(ui);
        });
    });
}
