#[derive(serde::Deserialize, serde::Serialize)]
//#[serde(default)]
pub struct FinItem {
    //date: Date
    item: String,
    category: String,
    price: f32,
    owner: String,
    ratio: f32
}


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,

    items: Vec<FinItem>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            items: Vec::new(),
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
        let Self { label, value, items } = self;

        // top (menu) bar
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // menu bar starting from left
                ui.menu_button("File", |ui| {
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

        // bottom panel
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.label("Hello World!");
        });

        // side panel
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            egui::warn_if_debug_build(ui);

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

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

        // central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanels and SidePanels
            ui.heading("Central Panel");

            if ui.button("Add Item").clicked() {
                let it = FinItem {
                    item: "a".to_string(),
                    category: "b".to_string(),
                    price: 0.0,
                    owner: "c".to_string(),
                    ratio: 1.0
                };

                items.insert(0, it);
            }

            // main grid
            // headers
            egui::Grid::new("main_grid").striped(true).show(ui, |ui| {

                // header
                //ui.label("Date");
                ui.label("Item");
                ui.label("Category");
                ui.label("Price");
                ui.label("Name");
                ui.label("Ratio");
                //ui.label("Total");
                ui.end_row();

                for row in items {
                    ui.label(&row.item);
                    ui.label(&row.category);
                    ui.label(&row.price.to_string());
                    ui.label(&row.owner);
                    ui.label(&row.ratio.to_string());
                    ui.end_row();
                }
            });




        });
    }

    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
