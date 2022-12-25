use chrono::Datelike;
use egui::plot::{Bar, BarChart, Legend, Plot};
use egui::{Response, Ui};
//use log::warn;

use crate::common::read_lines;
use crate::model::FinItem;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    items: Vec<FinItem>,

    // computed stuff:
    // this how you opt-out of serialization of a member
    #[serde(skip)]
    total: f32,
    #[serde(skip)]
    selected_year: i32,
    #[serde(skip)]
    selected_month: u32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            total: 0.0,
            selected_year: 2022,
            selected_month: 12,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn bar_stacked(ui: &mut Ui, items_in_month: &Vec<FinItem>, name: String) -> Response {
        let mut bars: Vec<Bar> = Vec::new();
        let mut cnt = 0;
        for item in items_in_month.iter() {
            bars.push(Bar::new(cnt as f64, item.price as f64).name(item.date.to_string()));
            cnt += 1;
        }
        let chart = BarChart::new(bars).width(0.7).name(name).vertical();

        Plot::new("Month")
            //.auto_bounds_x()
            //.auto_bounds_y()
            .legend(Legend::default())
            .allow_boxed_zoom(false)
            .allow_zoom(true)
            .allow_double_click_reset(true)
            .show(ui, |plot_ui| {
                //v = plot_ui.pointer_coordinate_drag_delta();
                //pp = plot_ui.pointer_coordinate();
                plot_ui.bar_chart(chart);
            })
            .response
    }
}

impl eframe::App for TemplateApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            items,
            total,
            selected_year,
            selected_month,
        } = self;

        // logic

        // I want tabs according to the months (year?)
        // sort by year then months
        let mut items_in_month: Vec<FinItem> = Vec::new();
        for item in items.iter() {
            if item.date.year() == *selected_year && item.date.month() == *selected_month {
                items_in_month.push(item.clone());
            }
        }
        // to calculate: for each item the calculated value
        // the monthly total
        let mut local_total = 0.0;
        for item in items_in_month.iter() {
            local_total += item.price;
        }
        *total = local_total;
        // the graphs?

        // top (menu) bar
        egui::TopBottomPanel::top("top_panel")
            .min_height(32.0)
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    // menu bar starting from left
                    ui.menu_button("File", |ui| {
                        // Import button
                        if ui.button("Import Data").clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("csv", &["csv"])
                                .set_directory("/")
                                .pick_file()
                            {
                                if let Some(picked_path) = Some(path.display().to_string()) {
                                    if let Ok(lines) = read_lines(picked_path) {
                                        // Consumes the iterator, returns an (Optional) String
                                        for line in lines.flatten() {
                                            // TODO parse and add to items
                                            println!("{}", line);
                                        }
                                    }
                                }
                            }
                        }
                        // Export button
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
                    });
                });
            });

        // left side panel
        egui::SidePanel::left("side_panel")
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Side Panel");

                egui::warn_if_debug_build(ui);

                // input
                ui.horizontal(|ui| {
                    ui.label("Year: ");
                    ui.add(egui::Slider::new(selected_year, 2019..=2022));
                });
                ui.horizontal(|ui| {
                    ui.label("Year: ");
                    ui.add(egui::Slider::new(selected_month, 1..=12));
                });

                // calculated values
                ui.horizontal(|ui| {
                    ui.label("Total: ");
                    ui.label(total.to_string());
                });

                // graphs

                // dbg

                // ui.horizontal(|ui| {
                //     ui.label("Write something: ");
                //     ui.text_edit_singleline(label);
                // });

                // ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
                // if ui.button("Increment").clicked() {
                //     *value += 1.0;
                // }

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
            });

        // bottom panel
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .min_height(0.0)
            .show(ctx, |ui| {
                TemplateApp::bar_stacked(ui, &items_in_month, selected_month.to_string());
            });

        // central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanels and SidePanels
            ui.heading("Central Panel");

            // main panel
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Add item button
                if ui.button("Add Item").clicked() {
                    items.push(FinItem {
                        date: chrono::offset::Local::now().date_naive(),
                        item: "a".to_string(),
                        category: "b".to_string(),
                        price: 0.0,
                        owner: "c".to_string(),
                        ratio: 1.0,
                        editable: false,
                    });
                }

                // main grid
                //egui::ScrollArea::horizontal().show(ui, |ui| {
                egui::Grid::new("main_grid").striped(true).show(ui, |ui| {
                    // header
                    ui.label("Date");
                    ui.label("Item");
                    ui.label("Category");
                    ui.label("Price");
                    ui.label("Name");
                    ui.label("Ratio");
                    //
                    ui.label("Total");
                    ui.end_row();

                    // add items
                    let rf = items_in_month.clone();
                    for mut r in rf {
                        r.price += 1.0;
                    }
                    for mut row in items {
                        // editable fields
                        if row.editable {
                            ui.label(&row.date.to_string()); //TODO
                            ui.text_edit_singleline(&mut row.item);
                            ui.text_edit_singleline(&mut row.category);
                            ui.add(egui::DragValue::new(&mut row.price).speed(0.1));
                            ui.text_edit_singleline(&mut row.owner);
                            ui.add(egui::Slider::new(&mut row.ratio, 0.0..=1.0));
                        } else {
                            ui.label(&row.date.to_string());
                            ui.label(&row.item);
                            ui.label(&row.category);
                            ui.label(&row.price.to_string());
                            ui.label(&row.owner);
                            ui.label(&row.ratio.to_string());
                        }

                        // calculated values
                        ui.label((row.price * row.ratio).to_string());

                        // edit button
                        let edit_button_text = if row.editable { "Stop editing" } else { "Edit" };
                        if ui.add(egui::Button::new(edit_button_text)).clicked() {
                            row.editable = !row.editable;
                        }
                        ui.end_row();
                    }
                });
            });
        });
    }

    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    //
}
