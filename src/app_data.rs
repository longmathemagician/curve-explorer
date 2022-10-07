use crate::{
    math::{bezier::Bezier, vector::Vector},
    APP_SIG,
};
use druid::{Data, Rect};
use preferences::Preferences;

#[derive(Clone, Data)]
pub struct AppData {
    #[data(ignore)]
    pub spline: Vec<Bezier<3>>,
    #[data(ignore)]
    pub offset: f32,
    pub viewport: Rect,
}

impl AppData {
    pub fn new() -> Self {
        let spline: Vec<Bezier<3>>;
        // let result = Vec::<Bezier<3>>::load(&APP_SIG, "saved_spline");

        // if let Ok(splines) = result {
        //     spline = splines.into();
        // } else {
        let curve_points = [
            Vector::<2>::new([0.2, 0.2]),
            Vector::<2>::new([0.1, 0.9]),
            Vector::<2>::new([0.4, 0.4]),
            Vector::<2>::new([0.9, 0.8]),
        ];
        spline = vec![Bezier::<3>::new(curve_points)];

        Self {
            spline,
            offset: 0.,
            viewport: Rect::ZERO,
        }
    }
}
