use super::vec2::Vec2;

#[derive(Clone)]
pub struct Bezier3 {
    pub control_points: Vec<Vec2<f32>>,
}
impl Bezier3 {
    pub fn new(points: Vec<Vec2<f32>>) -> Self {
        Self {
            control_points: points,
        }
    }

    // Hacky bounding box extraction
    pub fn _viewport(&self) -> [f32; 4] {
        let mut x_min = self.control_points[0].x;
        let mut y_min = self.control_points[0].y;
        let mut x_max = self.control_points[3].x;
        let mut y_max = self.control_points[3].y;

        for p in &self.control_points {
            if p.x < x_min {
                x_min = p.x
            }
            if p.x > x_max {
                x_max = p.x
            }
            if p.y < y_min {
                y_min = p.y
            }
            if p.y > y_max {
                y_max = p.y
            }
        }

        [x_min, y_min, x_max, y_max]
    }

    pub fn render(&self) -> Vec<Vec2<f32>> {
        self.render_naive()
    }

    fn render_by_arclen(&self) -> Vec<Vec2<f32>> {
        todo!()
    }

    fn render_naive(&self) -> Vec<Vec2<f32>> {
        let mut rendered_points: Vec<Vec2<f32>> = Vec::new();
        let n = self.control_points.len();

        for t in 0..25 + 1 {
            let t = t as f32 / 25.0;
            let mut p = Vec2::new(0.0, 0.0);
            for i in 0..n {
                let binomial = self.binomial_coefficient(n - 1, i);
                p.x += self.control_points[i].x
                    * binomial
                    * t.powi(i as i32)
                    * (1.0 - t).powi((n - 1 - i) as i32);
                p.y += self.control_points[i].y
                    * binomial
                    * t.powi(i as i32)
                    * (1.0 - t).powi((n - 1 - i) as i32);
            }
            rendered_points.push(p);
        }
        rendered_points
    }

    fn binomial_coefficient(&self, n: usize, k: usize) -> f32 {
        let mut result = 1.0;
        for i in 1..k + 1 {
            result *= ((n + 1) as f32 - i as f32) as f32 / i as f32;
        }
        result
    }
}
