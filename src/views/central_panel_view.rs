use egui_extras::Column;

use crate::{common::to_name, model::FinItem, TemplateApp};

pub(crate) fn show(ui: &mut egui::Ui, app: &mut TemplateApp, items_in_month: &[FinItem]) {
    // The central panel the region left after adding TopPanels and SidePanels
    ui.heading(to_name(app.selected_month));
    // main panel
    egui::ScrollArea::vertical().show(ui, |ui| {
        // Add item button
        if ui.button("Add Item").clicked() {
            app.items.push(FinItem {
                date: chrono::offset::Local::now().date_naive(),
                item: "item".to_string(),
                category: "category".to_string(),
                price: 0.0,
                owner: "MB".to_string(),
                ratio: 0.5,
                editable: false,
            });
        }

        // main grid
        let mut to_remove: Option<&FinItem> = None;
        egui_extras::TableBuilder::new(ui)
            .striped(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            //.column(Column::auto().at_least(40.0).resizable(true).clip(true)) // date
            .column(Column::auto()) // date
            .column(Column::auto()) // item
            .column(Column::auto()) // category
            .column(Column::auto()) // price
            .column(Column::auto()) // name
            .column(Column::auto()) // ratio
            .column(Column::auto()) // Total
            .column(Column::remainder()) // Options
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Date");
                });
                header.col(|ui| {
                    ui.strong("Item");
                });
                header.col(|ui| {
                    ui.strong("Category");
                });
                header.col(|ui| {
                    ui.strong("Price");
                });
                header.col(|ui| {
                    ui.strong("Name");
                });
                header.col(|ui| {
                    ui.strong("Ratio");
                });
                header.col(|ui| {
                    ui.strong("Total");
                });
                header.col(|ui| {
                    ui.strong("Options");
                });
            })
            .body(|mut body| {
                for row in app.items.iter_mut().filter(|i| items_in_month.contains(i)) {
                    body.row(18.0, |mut table_row| {
                        // editable fields
                        if row.editable {
                            table_row.col(|ui| {
                                ui.add(egui_extras::DatePickerButton::new(&mut row.date));
                            });
                            table_row.col(|ui| {
                                ui.text_edit_singleline(&mut row.item);
                            });
                            table_row.col(|ui| {
                                ui.text_edit_singleline(&mut row.category);
                            });
                            table_row.col(|ui| {
                                ui.add(egui::DragValue::new(&mut row.price).speed(0.1));
                            });
                            table_row.col(|ui| {
                                ui.text_edit_singleline(&mut row.owner);
                            });
                            table_row.col(|ui| {
                                ui.add(egui::Slider::new(&mut row.ratio, 0.0..=1.0));
                            });
                        } else {
                            table_row.col(|ui| {
                                ui.label(&row.date.to_string());
                            });
                            table_row.col(|ui| {
                                ui.label(&row.item);
                            });
                            table_row.col(|ui| {
                                ui.label(&row.category);
                            });
                            table_row.col(|ui| {
                                ui.label(&row.price.to_string());
                            });
                            table_row.col(|ui| {
                                ui.label(&row.owner);
                            });
                            table_row.col(|ui| {
                                ui.label(&row.ratio.to_string());
                            });
                        }

                        // calculated values
                        table_row.col(|ui| {
                            ui.label((row.price * row.ratio).to_string());
                        });

                        // edit button
                        table_row.col(|ui| {
                            let edit_button_text =
                                if row.editable { "Stop editing" } else { "Edit" };
                            if ui.add(egui::Button::new(edit_button_text)).clicked() {
                                row.editable = !row.editable;
                            }
                            //});
                            // delete button
                            //table_row.col(|ui| {
                            if ui.add(egui::Button::new("Delete")).clicked() {
                                // get current index
                                _ = to_remove.insert(row);
                            }
                        });
                    });
                }
            });

        // handle delete
        if let Some(c) = to_remove {
            let cc = c.to_owned();
            if let Some(pos) = app.items.iter().position(|x| *x == cc) {
                app.items.remove(pos);
            }
        }
    });
}
