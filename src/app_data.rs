use druid::{Data, Rect};

use crate::math::bezier3::Bezier3;

#[derive(Clone, Data)]
pub struct AppData {
    #[data(ignore)]
    pub curves: Vec<Bezier3>,
    pub viewport: Rect,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            curves: Vec::new(),
            viewport: Rect::ZERO,
        }
    }

    pub fn push_curve(&mut self, curve: Bezier3) {
        self.curves.push(curve);
    }
}
