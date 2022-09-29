use druid::{AppLauncher, Rect, WindowDesc};

mod app_data;
use app_data::*;

mod container_widget;
use container_widget::*;

mod math;
use math::{bezier3::Bezier3, vec2::Vec2};

fn main() {
    let mut data = AppData::new();

    // Create a curve and push it to the AppData instance
    let points = vec![
        Vec2::new(0.2, 0.2),
        Vec2::new(0.1, 0.9),
        Vec2::new(0.4, 0.4),
        Vec2::new(0.9, 0.8),
    ];
    data.push_curve(Bezier3::new(points));

    // Set the curvespace viewport
    data.viewport = Rect::new(0., 0., 1., 1.);

    // Launch the program
    let window = WindowDesc::new(ContainerWidget::new());
    AppLauncher::with_window(window)
        .log_to_console()
        .launch(data)
        .unwrap();
}
