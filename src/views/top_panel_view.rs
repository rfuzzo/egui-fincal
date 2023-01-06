use log::warn;
use rfd::FileDialog;
use std::{
    fs::File,
    io::{BufRead, BufReader, LineWriter, Write},
};

use crate::{model::FinItem, TemplateApp};

pub(crate) fn show(ui: &mut egui::Ui, _frame: &mut eframe::Frame, app: &mut TemplateApp) {
    egui::menu::bar(ui, |ui| {
        // menu bar starting from left
        ui.menu_button("File", |ui| {
            // Import button
            #[cfg(not(target_arch = "wasm32"))] // no File->Export on web pages!
            if ui.button("Import Data").clicked() {
                let file_option = rfd::FileDialog::new()
                    .add_filter("csv", &["csv"])
                    .set_directory("/")
                    .pick_file();

                if let Some(path) = file_option {
                    if let Ok(file) = File::open(path.as_path()) {
                        for (i, line) in BufReader::new(file).lines().enumerate() {
                            if let Some(item) = parse_line(line) {
                                app.items.push(item);
                            } else {
                                warn!("Failed to parse line {}", i)
                            }
                        }
                    }
                }
            }

            // Export button
            #[cfg(not(target_arch = "wasm32"))] // no File->Export on web pages!
            if ui.button("Export Data").clicked() {
                let some_path = FileDialog::new()
                    .add_filter("csv", &["csv"])
                    .set_directory("/")
                    .save_file();

                if let Some(path) = some_path {
                    if let Ok(file) = File::create(path.as_path()) {
                        let mut fw = LineWriter::new(file);
                        for i in app.items.iter() {
                            let str = format!("{i}\n");
                            match fw.write(str.as_bytes()) {
                                Ok(_) => {}
                                Err(e) => warn!("Failed to write line {}", e),
                            }
                        }
                    }
                }
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

fn parse_line(line: Result<String, std::io::Error>) -> Option<FinItem> {
    match line {
        Ok(s) => match s.as_str().parse::<FinItem>() {
            Ok(item) => Some(item),
            Err(_) => {
                warn!("Failed to parse line. Error:");
                None
            }
        },
        Err(err) => {
            warn!("Failed to read line {}", err);
            None
        }
    }
}
