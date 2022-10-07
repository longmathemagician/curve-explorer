#![feature(generic_const_exprs)]

use druid::{AppLauncher, Rect, WindowDesc};
use preferences::AppInfo;
mod app_data;
use app_data::*;
mod container_widget;
use container_widget::*;
mod app_delegate;
mod math;
use app_delegate::Delegate;

pub const APP_SIG: AppInfo = AppInfo {
    name: env!("CARGO_PKG_NAME"),
    author: env!("CARGO_PKG_AUTHORS"),
};

fn main() {
    let mut data = AppData::new();

    // Set the curve offset distance
    data.offset = 0.075;

    // Set the curvespace viewport
    data.viewport = Rect::new(0., 0., 1., 1.);

    // Launch the program
    let window = WindowDesc::new(ContainerWidget::new());
    AppLauncher::with_window(window)
        .delegate(Delegate::new())
        .log_to_console()
        .launch(data)
        .unwrap();
}
