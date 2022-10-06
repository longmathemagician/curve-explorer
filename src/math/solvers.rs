pub fn _newton(mut x: f32, f: impl Fn(f32) -> f32, df: impl Fn(f32) -> f32, tol: f32) -> f32 {
    for _ in 0..10 {
        let x_n = x - f(x) / df(x);
        if (x - x_n).abs() <= tol {
            return x_n;
        } else {
            x = x_n;
        }
    }
    x
}

pub fn newton2(
    mut x: f32,
    mut y: f32,
    f: impl Fn(f32, f32) -> f32,
    dfx: impl Fn(f32, f32) -> f32,
    dfy: impl Fn(f32, f32) -> f32,
    g: impl Fn(f32, f32) -> f32,
    dgx: impl Fn(f32, f32) -> f32,
    dgy: impl Fn(f32, f32) -> f32,
    tol: f32,
) -> (f32, f32) {
    for _ in 0..1000 {
        let d = dfx(x, y) * dgy(x, y) - dfy(x, y) * dgx(x, y);
        let x_n = x - (f(x, y) * dgy(x, y) - g(x, y) * dfy(x, y)) / d;
        let y_n = y - (g(x, y) * dfx(x, y) - f(x, y) * dgx(x, y)) / d;

        if (x - x_n).abs() <= tol && (y - y_n).abs() <= tol {
            return (x_n, y_n);
        } else {
            x = x_n;
            y = y_n;
        }
    }
    (x, y)
}
