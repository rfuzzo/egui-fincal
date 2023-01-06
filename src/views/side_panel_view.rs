use chrono::{self, Datelike};
use egui_extras::Column;
use itertools::Itertools;

use crate::common::to_name;
use crate::model::FinItem;
use crate::TemplateApp;

use std::collections::HashMap;

pub(crate) fn show(
    ui: &mut egui::Ui,
    app: &mut TemplateApp,
    items_in_month: &[FinItem],
    possible_years: &Vec<i32>,
    paid_dict: HashMap<String, (f32, f32)>,
) {
    ui.heading("Details");
    // inputs
    ui.group(|ui| {
        // year slider
        let mut first_year = chrono::offset::Local::now().date_naive().year();
        let mut last_year = first_year;
        if !possible_years.is_empty() {
            first_year = *possible_years.first().unwrap();
            last_year = *possible_years.last().unwrap();
        }
        ui.horizontal(|ui| {
            ui.label("Year: ");
            ui.add(egui::Slider::new(
                &mut app.selected_year,
                first_year..=last_year,
            ));
        });

        // months slider
        let month_str = to_name(app.selected_month);
        ui.horizontal(|ui| {
            ui.label("Month: ");
            ui.add(egui::Slider::new(&mut app.selected_month, 1..=12).text(month_str));
        });

        // categories
        let mut categories: String = app.categories.join(";");
        ui.horizontal(|ui| {
            ui.label("Categories: ");
            ui.text_edit_singleline(&mut categories);
        });
    });
    // calculated values
    // individual totals
    ui.group(|ui| {
        ui.vertical(|ui| {
            ui.label("Paid this month: ");
            ui.push_id("totals_table", |ui| {
                let totals_table = egui_extras::TableBuilder::new(ui)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::auto()) // name
                    .column(Column::auto().at_least(40.0).clip(true)) // price
                    .column(Column::remainder()) // owed
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.strong("Name");
                        });
                        header.col(|ui| {
                            ui.strong("Paid");
                        });
                        header.col(|ui| {
                            ui.strong("Owed");
                        });
                    });
                totals_table.body(|mut body| {
                    for key in paid_dict.keys().sorted() {
                        body.row(18.0, |mut row| {
                            row.col(|ui| {
                                ui.label(key);
                            });
                            // relative to ratio
                            row.col(|ui| {
                                ui.label(paid_dict[key].0.to_string());
                            });
                            // total
                            row.col(|ui| {
                                ui.label(paid_dict[key].1.to_string());
                            });
                        });
                    }
                });
            });

            ui.separator();

            // total
            ui.horizontal(|ui| {
                ui.label("Total spent: ");
                ui.label(app.total.to_string());
            });

            // owed totals
            if app.owners.len() > 1 {
                ui.separator();

                if app.owners_compare.0 == "None" || app.owners_compare.1 == "None" {
                    app.owners_compare = (app.owners[0].to_string(), app.owners[1].to_string());
                }

                ui.horizontal(|ui| {
                    // select main

                    egui::ComboBox::from_id_source("Main")
                        .selected_text(app.owners_compare.0.to_string())
                        .show_ui(ui, |ui| {
                            for o in &app.owners {
                                let oo = o.to_string();
                                ui.selectable_value(&mut app.owners_compare.0, oo, o);
                            }
                        });
                    // select to compare
                    egui::ComboBox::from_id_source("Compare")
                        .selected_text(app.owners_compare.1.to_string())
                        .show_ui(ui, |ui| {
                            for o in &app.owners {
                                let oo = o.to_string();
                                ui.selectable_value(&mut app.owners_compare.1, oo, o);
                            }
                        });
                });

                let main = paid_dict[app.owners_compare.0.as_str()].1;
                let compare = paid_dict[app.owners_compare.1.as_str()].1;
                let owed = compare - main;
                ui.horizontal(|ui| {
                    ui.label("Total owed: ");
                    ui.label(owed.to_string());
                });
            }
        });
    });
    // by category
    ui.group(|ui| {
        ui.vertical(|ui| {
            ui.label("Spent by category: ");
            ui.push_id("category_table", |ui| {
                let category_table = egui_extras::TableBuilder::new(ui)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::auto()) // name
                    .column(Column::remainder()) // owed
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.strong("Category");
                        });
                        header.col(|ui| {
                            ui.strong("Paid");
                        });
                    });

                // order items by category and sum them up
                let cat_dict = items_in_month
                    .iter()
                    .map(|i| {
                        let x = i
                            .category
                            .as_ref()
                            .unwrap_or(&"None".to_string())
                            .to_string();
                        (x, i.price)
                    })
                    .into_group_map()
                    .iter()
                    .map(|(k, v)| {
                        let result: f32 = v.iter().sum();
                        (k.to_string(), result)
                    })
                    .collect::<HashMap<_, _>>();

                // view table
                category_table.body(|mut body| {
                    // print categories
                    for key in cat_dict.keys().sorted() {
                        body.row(18.0, |mut row| {
                            row.col(|ui| {
                                ui.label(key.to_string());
                            });
                            row.col(|ui| {
                                ui.label(cat_dict[key].to_string());
                            });
                        });
                    }
                });
            });
        });
    });
    // footer
    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label("powered by ");
            ui.hyperlink_to("egui", "https://github.com/emilk/egui");
            ui.label(" and ");
            ui.hyperlink_to(
                "eframe",
                "https://github.com/emilk/egui/tree/master/crates/eframe",
            );
            ui.label(".");
        });
    });
}
