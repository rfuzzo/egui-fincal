use chrono::{Datelike, Month};
use egui::plot::{Bar, BarChart, HLine, Legend, Line, Plot, PlotPoints};
use num_traits::FromPrimitive;
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
            selected_month: chrono::offset::Local::now().date_naive().month(),
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
        // sort by year then months
        let mut items_in_month: Vec<FinItem> = Vec::new();
        let mut possible_years: Vec<i32> = Vec::new();
        //let mut possible_months: Vec<u32> = Vec::new();

        for item in items.iter() {
            let year = item.date.year();
            let month = item.date.month();

            if !possible_years.contains(&year) {
                possible_years.push(year);
            }

            // if !possible_months.contains(&month) {
            //     possible_months.push(month);
            // }

            if year == *selected_year && month == *selected_month {
                items_in_month.push(item.clone());
            }
        }

        possible_years.sort();
        //possible_months.sort();

        // to calculate: for each item the calculated value
        // the monthly total
        let mut local_total = 0.0;
        for item in items_in_month.iter() {
            local_total += item.price;
        }
        *total = local_total;

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
                ui.heading("Details");

                egui::warn_if_debug_build(ui);

                // input
                let mut first_year = chrono::offset::Local::now().date_naive().year();
                let mut last_year = first_year;
                if possible_years.len() > 0 {
                    first_year = *possible_years.first().unwrap();
                    last_year = *possible_years.last().unwrap();
                }

                ui.horizontal(|ui| {
                    ui.label("Year: ");
                    ui.add(egui::Slider::new(selected_year, first_year..=last_year));
                });

                let month_str = to_name(*selected_month);
                ui.horizontal(|ui| {
                    ui.label("Month: ");
                    ui.add(egui::Slider::new(selected_month, 1..=12).text(month_str));
                });

                // calculated values
                ui.horizontal(|ui| {
                    ui.label("Total: ");
                    ui.label(total.to_string());
                });

                // graphs

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
            });

        // bottom panel
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .min_height(100.0)
            .show(ctx, |ui| {
                let mut bars: Vec<Bar> = Vec::new();
                let mut dots: Vec<f64> = Vec::new();
                let mut cnt = 0;
                for item in items_in_month.iter() {
                    bars.push(Bar::new(cnt as f64, item.price as f64).name(item.date.to_string()));
                    dots.push(item.price as f64);
                    //total += item.price;
                    cnt += 1;
                }

                // Get daily expenses as bars
                let bar_chart = BarChart::new(bars)
                    .width(0.7)
                    .name(to_name(*selected_month))
                    .vertical();

                // Get daily expenses as line
                let plot_points: PlotPoints = dots
                    .iter()
                    .enumerate()
                    .map(|(i, &y)| [i as f64, y as f64])
                    .collect();
                let line = Line::new(plot_points)
                    .color(egui::Color32::from_rgb(100, 200, 100))
                    .name(to_name(*selected_month));

                // Get daily expenses as average line
                let hline = HLine::new(*total / items_in_month.len() as f32)
                    .name("Average")
                    .highlight(true);

                // construct plot
                let plot = Plot::new("Month")
                    .reset()
                    .legend(Legend::default())
                    .allow_boxed_zoom(false)
                    .allow_zoom(false)
                    .allow_drag(false)
                    .allow_double_click_reset(true)
                    .auto_bounds_x()
                    .auto_bounds_y();

                // draw plot
                plot.show(ui, |plot_ui| {
                    plot_ui.bar_chart(bar_chart);

                    plot_ui.line(line);

                    plot_ui.hline(hline);
                });
            });

        // central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanels and SidePanels
            ui.heading(to_name(*selected_month));

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
                    for mut row in items_in_month {
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
}

/// Get English name of month for index
fn to_name(month_idx: u32) -> String {
    let some_month = Month::from_u32(month_idx);
    match some_month {
        Some(month) => month.name().to_owned(),
        None => "".to_owned(),
    }
}
