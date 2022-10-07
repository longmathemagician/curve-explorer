use super::vector::Vector;

#[derive(Clone)]
pub struct Bezier<const N: usize>
where
    [(); N + 1]:,
{
    pub p: [Vector<2>; N + 1],
}

impl<const N: usize> Bezier<N>
where
    [(); N + 1]:,
{
    /// Returns a new Bezier curve from the provided point array.
    pub fn new(p: [Vector<2>; N + 1]) -> Self {
        Self { p }
    }

    /// Returns an oriented convex hull for the curve.
    pub fn convex_hull(&self) -> [Vector<2>; N + 1] {
        // Jarvis march-ish algorithm, probably in its worst-case scenario

        // Find a corner point (bottom left in this case)
        let mut i_start = 0;
        for i in 0..N + 1 {
            if self.p[i].b[0] < self.p[i_start].b[0] {
                i_start = i;
            } else if self.p[i].b[0] == self.p[i_start].b[0] {
                if self.p[i].b[1] < self.p[i_start].b[1] {
                    i_start = i;
                }
            }
        }

        // Hacky dual-stack point sort
        let mut unsorted: Vec<usize> = (0..N + 1).collect();
        let mut sorted: Vec<usize> = vec![unsorted.swap_remove(i_start)];

        while !unsorted.is_empty() {
            // Compute the angle between the start point and the remaining points
            let j = sorted.last().unwrap();
            let mut max_theta = 0.;
            let mut next_point = 0;

            // Iterate through remaining points and find the one with the largest
            // angle to the current endpoint
            for v in unsorted.iter().enumerate() {
                let phi = (self.p[*v.1].b[0] - self.p[*j].b[0])
                    .atan2(self.p[*v.1].b[1] - self.p[*j].b[1]);
                if phi > max_theta {
                    max_theta = phi;
                    next_point = v.0;
                }
            }

            // Swap the point from the unsorted stack to the sorted stack
            sorted.push(unsorted.swap_remove(next_point));
        }

        // Return ordered control points
        let mut hull: [Vector<2>; N + 1] = [Vector::<2>::ZERO; N + 1];
        for i in 0..N + 1 {
            hull[i] = self.p[sorted[i]];
        }
        hull
    }

    pub fn get_point(&self, t: f64) -> Vector<2> {
        let mut point: Vector<2> = Vector::new([0., 0.]);
        for i in 0..N + 1 {
            let poly = self.bernstein_polynomial(t, i);
            point.b[0] += self.p[i].b[0] * poly;
            point.b[1] += self.p[i].b[1] * poly;
        }
        point
    }

    fn bernstein_polynomial(&self, t: f64, i: usize) -> f64 {
        self.binomial_coefficient(i) * t.powi(i as i32) * (1.0 - t).powi((N - i) as i32)
    }

    fn binomial_coefficient(&self, k: usize) -> f64 {
        let mut result = 1.;
        let n_f = (N + 1) as f64;
        for i in 1..k + 1 {
            result *= (i as f64).recip() * n_f - 1.;
        }
        result
    }

    pub fn start_tangent(&self) -> Vector<2> {
        assert_eq!(N, 3); // For cubic beziers only
        3. * (self.p[1] - self.p[0])
    }

    pub fn end_tangent(&self) -> Vector<2> {
        assert_eq!(N, 3);
        3. * (self.p[3] - self.p[2])
    }

    pub fn first_derivative(&self, t: f64) -> Vector<2> {
        assert_eq!(N, 3);
        3. * (1. - t).powi(2) * (self.p[1] - self.p[0])
            + 6. * (1. - t) * t * (self.p[2] - self.p[1])
            + 3. * t.powi(2) * (self.p[3] - self.p[2])
    }

    pub fn second_derivative(&self, t: f64) -> Vector<2> {
        assert_eq!(N, 3);
        6. * (1. - t) * (self.p[2] - 2. * self.p[1] + self.p[0])
            + 6. * t * (self.p[3] - 2. * self.p[2] + self.p[1])
    }

    pub fn curvature(&self, t: f64) -> f64 {
        assert_eq!(N, 3);
        (self.second_derivative(t).cross(&self.first_derivative(t)))
            / self.first_derivative(t).magnitude().powi(3)
    }

    pub fn flatten(&self, tol: f64) -> Vec<Vector<2>> {
        let mut rendered_points: Vec<Vector<2>> = Vec::new();

        for t in 0..25 + 1 {
            let t = t as f64 / 25.0;
            rendered_points.push(self.get_point(t));
        }
        rendered_points
    }
}
