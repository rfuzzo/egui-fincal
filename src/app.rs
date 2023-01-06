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
    pub items: Vec<FinItem>,
    pub categories: Vec<String>,

    // computed stuff:
    // this how you opt-out of serialization of a member
    #[serde(skip)]
    pub total: f32,
    #[serde(skip)]
    pub selected_year: i32,
    #[serde(skip)]
    pub selected_month: u32,
    #[serde(skip)]
    pub owners: Vec<String>,
    #[serde(skip)]
    pub owners_compare: (String, String),
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            categories: vec!["a".to_string(), "b".into(), "c".into()],

            // calculated
            total: 0.0,
            selected_year: chrono::offset::Local::now().date_naive().year(),
            selected_month: chrono::offset::Local::now().date_naive().month(),
            owners: Vec::new(),
            owners_compare: ("None".to_owned(), "None".to_owned()),
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

            // calculated
            total,
            selected_year,
            selected_month,
            owners,
            ..
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

        // select correct year
        if !possible_years.contains(selected_year) {
            *selected_year = *possible_years.last().unwrap();
        }

        // to calculate: for each item the calculated value
        owners.clear();
        *total = 0.0;
        let mut paid_dict: HashMap<String, (f32, f32)> = HashMap::new();
        for item in items_in_month.iter() {
            // the monthly total
            *total += item.price;
            let key = &item.owner;
            // add to owners dict
            if !owners.contains(key) {
                owners.push(key.to_string());
            }

            // the monthly total for each name
            let partial_price = item.price * item.ratio;
            if paid_dict.contains_key(key.as_str()) {
                paid_dict.entry(key.to_string()).and_modify(|(v1, v2)| {
                    *v1 += item.price;
                    *v2 += partial_price;
                });
            } else {
                paid_dict
                    .entry(key.to_string())
                    .or_insert((item.price, partial_price));
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
                views::top_panel_view::show(ui, _frame, &mut *self);
            });

        ////////////////////////////////
        // left side panel
        egui::SidePanel::left("side_panel")
            .resizable(true)
            .show(ctx, |ui| {
                views::side_panel_view::show(
                    ui,
                    &mut *self,
                    &items_in_month,
                    &possible_years,
                    paid_dict,
                );
            });

        ////////////////////////////////
        // bottom panel
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .min_height(100.0)
            .show(ctx, |ui| {
                views::bottom_panel_view::show(ui, &mut *self, &items_in_month);
            });

        ////////////////////////////////
        // central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            views::central_panel_view::show(ui, &mut *self, &items_in_month, &possible_years);
        });
    }

    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
