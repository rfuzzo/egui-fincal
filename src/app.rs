use chrono::Datelike;
use std::collections::HashMap;

// local
use crate::model::FinItem;
use crate::views;

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

        ////////////////////////////////
        // logic
        ////////////////////////////////

        // sort by year then months
        let mut items_in_month: Vec<FinItem> = Vec::new();
        let mut possible_years: Vec<i32> = Vec::new();
        for item in items.iter() {
            let year = item.date.year();
            let month = item.date.month();

            if !possible_years.contains(&year) {
                possible_years.push(year);
            }

            if year == *selected_year && month == *selected_month {
                items_in_month.push(item.clone());
            }
        }
        possible_years.sort();

        // to calculate: for each item the calculated value
        *total = 0.0;
        let mut paid_dict: HashMap<&str, (f32, f32)> = HashMap::new();
        for item in items_in_month.iter() {
            // the monthly total
            *total += item.price;
            let key = &item.owner.as_str();
            // the monthly total for each name
            let partial_price = item.price * item.ratio;
            if paid_dict.contains_key(key) {
                paid_dict.entry(key).and_modify(|(v1, v2)| {
                    *v1 += item.price;
                    *v2 += partial_price;
                });
            } else {
                paid_dict.entry(key).or_insert((item.price, partial_price));
            }
        }

        ////////////////////////////////
        // Layouts
        ////////////////////////////////

        ////////////////////////////////
        // top (menu) bar
        egui::TopBottomPanel::top("top_panel")
            .min_height(32.0)
            .show(ctx, |ui| {
                views::top_panel_view::show(ui, items, _frame);
            });

        ////////////////////////////////
        // left side panel
        egui::SidePanel::left("side_panel")
            .resizable(true)
            .show(ctx, |ui| {
                views::side_panel_view::show(
                    ui,
                    possible_years,
                    selected_year,
                    selected_month,
                    paid_dict,
                    total,
                    &items_in_month,
                );
            });

        ////////////////////////////////
        // bottom panel
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .min_height(100.0)
            .show(ctx, |ui| {
                views::bottom_panel_view::show(&items_in_month, selected_month, total, ui);
            });

        ////////////////////////////////
        // central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            views::central_panel_view::show(ui, selected_month, items, items_in_month);
        });
    }

    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
