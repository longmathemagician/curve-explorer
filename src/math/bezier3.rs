use super::{constants, solvers::newton2, vec2::Vec2};
use crate::math::matrix2x2::Matrix2x2;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Bezier3 {
    pub control_points: Vec<Vec2<f32>>,
}
impl Bezier3 {
    pub fn new(points: Vec<Vec2<f32>>) -> Self {
        Self {
            control_points: points,
        }
    }

    pub fn _hull(&self) -> [f32; 4] {
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

    pub fn flatten(&self, _tolerance: f32) -> Vec<Vec2<f32>> {
        self.render_naive()
    }

    fn render_naive(&self) -> Vec<Vec2<f32>> {
        let mut rendered_points: Vec<Vec2<f32>> = Vec::new();
        let n = self.control_points.len();

        for t in 0..25 + 1 {
            let t = t as f32 / 25.0;
            let mut p: Vec2<f32> = Vec2::new(0.0, 0.0);
            for i in 0..n {
                p.x += self.control_points[i].x * self.bernstein_polynomial(t, n, i);
                p.y += self.control_points[i].y * self.bernstein_polynomial(t, n, i);
            }
            rendered_points.push(p);
        }
        rendered_points
    }

    pub fn get_point(&self, t: f32) -> Vec2<f32> {
        let n = self.control_points.len();
        let mut point: Vec2<f32> = Vec2::new(0., 0.);
        for i in 0..n {
            point.x += self.control_points[i].x * self.bernstein_polynomial(t, n, i);
            point.y += self.control_points[i].y * self.bernstein_polynomial(t, n, i);
        }
        point
    }

    fn bernstein_polynomial(&self, t: f32, n: usize, i: usize) -> f32 {
        self.binomial_coefficient(n - 1, i) * t.powi(i as i32) * (1.0 - t).powi((n - 1 - i) as i32)
    }

    fn binomial_coefficient(&self, n: usize, k: usize) -> f32 {
        let mut result = 1.0;
        for i in 1..k + 1 {
            result *= ((n + 1) as f32 - i as f32) as f32 / i as f32;
        }
        result
    }

    pub fn first_derivative(&self, t: f32) -> Vec2<f32> {
        3. * (1. - t).powi(2) * (self.control_points[1] - self.control_points[0])
            + 6. * (1. - t) * t * (self.control_points[2] - self.control_points[1])
            + 3. * t.powi(2) * (self.control_points[3] - self.control_points[2])
    }

    pub fn second_derivative(&self, t: f32) -> Vec2<f32> {
        6. * (1. - t)
            * (self.control_points[2] - 2 * self.control_points[1] + self.control_points[0])
            + 6. * t
                * (self.control_points[3] - 2 * self.control_points[2] + self.control_points[1])
    }

    pub fn tangent(&self, t: f32) -> Vec2<f32> {
        self.first_derivative(t)
    }

    pub fn offset_klass(&self, d: f32) -> Self {
        // Convert to hermite form
        let t_0 = 3. * (self.control_points[1] - self.control_points[0]);
        let t_1 = 3. * (self.control_points[3] - self.control_points[2]);
        let p_0 = self.control_points[0];
        let p_1 = self.control_points[3];

        // Todo: Refine initial values with evaluation of f_l, g_l
        let c_0 = 1.;
        let c_1 = 1.;

        let alpha_0 = (1. / (1. / self.curvature(0.) - d)) * t_0.mag().powi(3);
        let alpha_1 = (1. / (1. / self.curvature(1.) - d)) * t_1.mag().powi(3);

        let n_0 = 2. * (t_0 / t_1) / alpha_0;
        let n_1 = 2. * (t_0 / t_1) / alpha_1;
        let w_0 = 6. * (((p_1 - p_0) / t_0) + d * (t_0 * (t_1 - t_0))) / alpha_0;
        let w_1 = 6. * (((p_0 - p_1) / t_1) + d * (t_1 * (t_0 - t_1))) / alpha_1;

        let f = |a: f32, b: f32| -> f32 { -b.powi(2) + n_0 * a + w_0 };
        let dfa = |_a: f32, _b: f32| -> f32 { n_0 };
        let dfb = |_a: f32, b: f32| -> f32 { -2. * b };
        let g = |a: f32, b: f32| -> f32 { -a.powi(2) + n_1 * b + w_1 };
        let dga = |a: f32, _b: f32| -> f32 { -2. * a };
        let dgb = |_a: f32, _b: f32| -> f32 { n_1 };

        let tol = 1e-7;

        let (c_0, c_1) = newton2(c_0, c_1, f, dfa, dfb, g, dga, dgb, tol);

        // Return as a cubic bezier
        let p_0 = p_0 + d * t_0.normal().normalize();
        let p_3 = p_1 + d * t_1.normal().normalize();
        let p_1 = p_0 + (t_0 * c_0) / 3.;
        let p_2 = p_3 - (t_1 * c_1) / 3.;
        Bezier3::new(vec![p_0, p_1, p_2, p_3])
    }

    pub fn offset_tiller(&self, d: f32) -> Self {
        let mut offset_curve: Vec<Vec2<f32>> = Vec::new();

        // Offset the first control point by the magnitude d and direction normal to the tangent
        // vector at the source curve's first control point
        offset_curve.push(self.control_points[0] + d * self.tangent(0.).normal().normalize());

        for i in 0..2 {
            let p0 = self.control_points[0 + i]
                + d * (self.control_points[1 + i] - self.control_points[0 + i])
                    .normal()
                    .normalize();
            let p1 = self.control_points[1 + i]
                + d * (self.control_points[1 + i] - self.control_points[0 + i])
                    .normal()
                    .normalize();
            let p2 = self.control_points[1 + i]
                + d * (self.control_points[2 + i] - self.control_points[1 + i])
                    .normal()
                    .normalize();
            let p3 = self.control_points[2 + i]
                + d * (self.control_points[2 + i] - self.control_points[1 + i])
                    .normal()
                    .normalize();

            let a1 = p1.y - p0.y;
            let b1 = p0.x - p1.x;
            let c1 = a1 * p0.x + b1 * p0.y;
            let a2 = p2.y - p3.y;
            let b2 = p3.x - p2.x;
            let c2 = a2 * p3.x + b2 * p3.y;
            let det = a1 * b2 - a2 * b1;
            assert_ne!(det, 0.);
            let x = (b2 * c1 - b1 * c2) / det;
            let y = (a1 * c2 - a2 * c1) / det;
            offset_curve.push(Vec2::new(x, y));
        }

        // As with the first control point, offset the final control point by the magnitude d and
        // direction normal to the tangent vector at the source curve's final control point
        offset_curve.push(self.control_points[3] + d * self.tangent(1.).normal().normalize());

        // Return the offset curve
        Bezier3::new(offset_curve)
    }

    pub fn curvature(&self, t: f32) -> f32 {
        (self.second_derivative(t) / self.first_derivative(t))
            / self.first_derivative(t).mag().powi(3)
    }

    pub fn offset_levien(&self, d: f32) -> Bezier3 {
        let affine: Matrix2x2<f32> = Matrix2x2::identity();
        // Move curve start to origin. With a 2x2 affine matrix we have to store an offset vector.
        let offset = self.control_points[0];

        // Rotate the curve so that the endpoint lies on the x-axis
        let l = self.control_points[3] - offset;
        let theta = l.y.atan2(l.x);
        let affine = affine.rotation(-theta);

        // Scale curve so that the endpoint is at (1, 0)
        let affine = affine.scale(1. / (affine * l).mag());

        // Create a new bezier object for convenience
        let mut normalized_curve_points: Vec<Vec2<f32>> = Vec::new();
        for p in &self.control_points {
            normalized_curve_points.push((affine * (*p - offset)));
        }
        let normalized_curve = Bezier3::new(normalized_curve_points);

        let th_0 = normalized_curve.control_points[1]
            .y
            .atan2(normalized_curve.control_points[1].x);
        let th_1 = normalized_curve.control_points[2]
            .y
            .atan2(1. - normalized_curve.control_points[2].x);

        let g_l = constants::GAUSS_LEGENDRE_COEFFS_32;
        let mut area = 0.;
        let mut x_moment = 0.;
        for i in (0..g_l.len()).step_by(2) {
            let w_i = g_l[i];
            let x_i = 0.5 * (1. + g_l[i + 1]);
            let p = normalized_curve.get_point(x_i);
            let dp = normalized_curve.first_derivative(x_i);
            let da = w_i * dp.x * p.y;
            area += da;
            x_moment += p.x * da;
        }

        area /= 2.;
        x_moment /= 2.;

        normalized_curve
    }
}
