use egui::plot::{Bar, BarChart, HLine, Legend, Line, Plot, PlotPoints};

use crate::{common::to_name, model::FinItem};

pub(crate) fn show(
    items_in_month: &Vec<FinItem>,
    selected_month: &mut u32,
    total: &mut f32,
    ui: &mut egui::Ui,
) {
    let mut bars: Vec<Bar> = Vec::new();
    let mut dots: Vec<f64> = Vec::new();
    for (cnt, item) in items_in_month.iter().enumerate() {
        bars.push(Bar::new(cnt as f64, item.price as f64).name(item.date.to_string()));
        dots.push(item.price as f64);
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
        .map(|(i, &y)| [i as f64, y])
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
}
