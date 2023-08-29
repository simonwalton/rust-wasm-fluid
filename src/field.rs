use crate::util::{lerp, xy, clamp, clamp_ab};

pub fn diffuse(x: &mut [f32], x0: &[f32], dt: f32, iter: u32, n: u32) {
    let diff = 0.005f32;
    let a = dt * diff * (n * n) as f32;

    for _k in 0..iter {
        for j in 1..n - 1 {
            for i in 1..n - 1 {
                let neighbours = x[xy(i - 1, j, n)]
                    + x[xy(i + 1, j, n)]
                    + x[xy(i, j - 1, n)]
                    + x[xy(i, j + 1, n)];
                x[xy(i, j, n)] =
                    clamp((x0[xy(i, j, n)] + a * neighbours) / (1.0f32 + 4.0f32 * a));
            }
        }
        set_boundary(x, BoundaryAction::Neighbour, n)
    }
}

pub fn advect(d: &mut [f32], d0: &[f32], u: &[f32], v: &[f32], dt: f32, n: u32) {
    let dt0 = dt * n as f32;

    for j in 1..n - 1 {
        for i in 1..n - 1 {
            let xp = clamp_ab(i as f32 - dt0 * u[xy(i, j, n)], 1.5f32, n as f32 - 2.5f32);
            let yp = clamp_ab(j as f32 - dt0 * v[xy(i, j, n)], 1.5f32, n as f32 - 2.5f32);
            let x0 = xp.floor() as u32;
            let x1 = x0 + 1;
            let y0 = yp.floor() as u32;
            let y1 = y0 + 1;

            d[xy(i, j, n)] = lerp(
                lerp(d0[xy(x0, y0, n)], d0[xy(x0, y1, n)], yp - y0 as f32),
                lerp(d0[xy(x1, y0, n)], d0[xy(x1, y1, n)], yp - y0 as f32),
                xp - x0 as f32,
            )
        }
    }

    set_boundary(d, BoundaryAction::Neighbour, n)
}

pub fn project(
    u: &mut [f32],
    v: &mut [f32],
    p: &mut [f32],
    d: &mut [f32],
    iter: u32,
    n: u32,
) {
    let h = 1.0f32 / n as f32;

    for j in 1..n - 1 {
        for i in 1..n - 1 {
            d[xy(i, j, n)] = -0.5
                * h
                * (u[xy(i + 1, j, n)] - u[xy(i - 1, j, n)] + v[xy(i, j + 1, n)]
                    - v[xy(i, j - 1, n)]);
            p[xy(i, j, n)] = 0.0f32;
        }
    }
    set_boundary(d, BoundaryAction::Neighbour, n);
    set_boundary(p, BoundaryAction::Neighbour, n);

    for _k in 0..iter {
        for j in 1..n - 1 {
            for i in 1..n - 1 {
                p[xy(i, j, n)] = (d[xy(i, j, n)]
                    + p[xy(i - 1, j, n)]
                    + p[xy(i + 1, j, n)]
                    + p[xy(i, j - 1, n)]
                    + p[xy(i, j + 1, n)])
                    / 4.0f32;
            }
        }
        set_boundary(p, BoundaryAction::Neighbour, n)
    }

    for j in 1..n - 1 {
        for i in 1..n - 1 {
            u[xy(i, j, n)] -= 0.5 * (p[xy(i + 1, j, n)] - p[xy(i - 1, j, n)]) / h;
            v[xy(i, j, n)] -= 0.5 * (p[xy(i, j + 1, n)] - p[xy(i, j - 1, n)]) / h;
        }
    }

    set_boundary(u, BoundaryAction::NegativeX, n);
    set_boundary(v, BoundaryAction::NegativeY, n);
}

#[derive(PartialEq)]
pub enum BoundaryAction {
    Neighbour,
    NegativeX,
    NegativeY,
}

pub fn set_boundary(x: &mut [f32], action: BoundaryAction, n: u32) {
    let xm = match action {
        BoundaryAction::NegativeX => -1.0f32,
        _ => 1.0f32,
    };
    let ym = match action {
        BoundaryAction::NegativeY => -1.0f32,
        _ => 1.0f32,
    };

    for i in 1..n {
        x[xy(0, i, n)] = xm * x[xy(1, i, n)];
        x[xy(n - 1, i, n)] = xm * x[xy(n - 2, i, n)];
        x[xy(i, 0, n)] = ym * x[xy(i, 1, n)];
        x[xy(i, n - 1, n)] = ym * x[xy(i, n - 2, n)];
    }
    x[xy(0, 0, n)] = 0.5 * (x[xy(1, 0, n)] + x[xy(0, 1, n)]);
    x[xy(n - 1, 0, n)] = 0.5 * (x[xy(n - 2, 0, n)] + x[xy(n - 1, 1, n)]);
    x[xy(0, n - 1, n)] = 0.5 * (x[xy(0, n - 2, n)] + x[xy(1, n - 1, n)]);
    x[xy(n - 1, n - 1, n)] = 0.5 * (x[xy(n - 1, n - 2, n)] + x[xy(n - 2, n - 1, n)]);
}