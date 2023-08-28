use std::u32;
use js_sys::Float32Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Fluid {
    dt: f32,            // time difference per tick
    iterations: u32,    // number of iterations for diffusion and projection steps
    resolution: u32,    // grid resolution (n*n; square)
    u: Vec<f32>,        // vector field u component
    u0: Vec<f32>,       // vector field u component (alternate)
    v: Vec<f32>,        // vector field v component
    v0: Vec<f32>,       // vector field v component (alternate)
    d: Vec<f32>,        // density field
    d0: Vec<f32>,       // density field (alternate)
}

fn add_array(a: &mut [f32], b: &[f32]) {
    for i in 0..a.len() {
        a[i] += b[i];
    }
}

#[inline(always)]
fn addr(x: u32, y: u32, n: u32) -> usize {
    (x + (y * n)) as usize
}

#[inline(always)]
fn clamp(x: f32) -> f32 {
    x.max(0.0f32).min(1.0f32)
}

#[inline(always)]
fn clamp_ab(x: f32, a: f32, b: f32) -> f32 {
    x.max(a).min(b)
}

#[inline(always)]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + ((b - a) * t)
}

#[derive(PartialEq)]
enum BoundaryAction {
    Neighbour,
    NegativeX,
    NegativeY,
}

#[wasm_bindgen]
impl Fluid {
    pub fn new(resolution: u32) -> Fluid {
        let dt = 0.001f32;
        let iterations = 10;
        let area = resolution.pow(2) as usize;
        
        // density
        let d = vec![0.0f32; area];
        let d0: Vec<f32> = vec![0.0f32; area];
        // velocity u
        let u = vec![0.0f32; area];
        let u0 = vec![0.0f32; area];
        // velocity v
        let v = vec![0.0f32; area];
        let v0 = vec![0.0f32; area];

        Fluid {
            dt,
            iterations,
            resolution,
            u,
            u0,
            v,
            v0,
            d,
            d0,
        }
    }

    pub fn set_dt(&mut self, dt: f32) {
        self.dt = dt;
    }

    pub fn set_iterations(&mut self, iterations: u32) {
        self.iterations = iterations;
    }

    pub fn d(&self) -> Float32Array {
        unsafe { Float32Array::view(&self.d) }
    }

    pub fn d0(&self) -> Float32Array {
        unsafe { Float32Array::view(&self.d0) }
    }

    pub fn source_u(&self) -> Float32Array {
        unsafe { Float32Array::view(&self.u0) }
    }

    pub fn source_v(&self) -> Float32Array {
        unsafe { Float32Array::view(&self.v0) }
    }

    fn diffuse(x: &mut [f32], x0: &[f32], dt: f32, iter: u32, n: u32) {
        let diff = 0.005f32;
        let a = dt * diff * (n * n) as f32;

        for _k in 0..iter {
            for j in 1..n - 1 {
                for i in 1..n - 1 {
                    let neighbours = x[addr(i - 1, j, n)]
                        + x[addr(i + 1, j, n)]
                        + x[addr(i, j - 1, n)]
                        + x[addr(i, j + 1, n)];
                    x[addr(i, j, n)] =
                        clamp((x0[addr(i, j, n)] + a * neighbours) / (1.0f32 + 4.0f32 * a));
                }
            }
            Fluid::set_boundary(x, BoundaryAction::Neighbour, n)
        }
    }

    fn advect(d: &mut [f32], d0: &[f32], u: &[f32], v: &[f32], dt: f32, n: u32) {
        let dt0 = dt * n as f32;

        for j in 1..n - 1 {
            for i in 1..n - 1 {
                let xp = clamp_ab(i as f32 - dt0 * u[addr(i, j, n)], 1.5f32, n as f32 - 2.5f32);
                let yp = clamp_ab(j as f32 - dt0 * v[addr(i, j, n)], 1.5f32, n as f32 - 2.5f32);
                let x0 = xp.floor() as u32;
                let x1 = x0 + 1;
                let y0 = yp.floor() as u32;
                let y1 = y0 + 1;

                d[addr(i, j, n)] = lerp(
                    lerp(d0[addr(x0, y0, n)], d0[addr(x0, y1, n)], yp - y0 as f32),
                    lerp(d0[addr(x1, y0, n)], d0[addr(x1, y1, n)], yp - y0 as f32),
                    xp - x0 as f32,
                )
            }
        }

        Fluid::set_boundary(d, BoundaryAction::Neighbour, n)
    }

    fn project(
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
                d[addr(i, j, n)] = -0.5
                    * h
                    * (u[addr(i + 1, j, n)] - u[addr(i - 1, j, n)] + v[addr(i, j + 1, n)]
                        - v[addr(i, j - 1, n)]);
                p[addr(i, j, n)] = 0.0f32;
            }
        }
        Fluid::set_boundary(d, BoundaryAction::Neighbour, n);
        Fluid::set_boundary(p, BoundaryAction::Neighbour, n);

        for _k in 0..iter {
            for j in 1..n - 1 {
                for i in 1..n - 1 {
                    p[addr(i, j, n)] = (d[addr(i, j, n)]
                        + p[addr(i - 1, j, n)]
                        + p[addr(i + 1, j, n)]
                        + p[addr(i, j - 1, n)]
                        + p[addr(i, j + 1, n)])
                        / 4.0f32;
                }
            }
            Fluid::set_boundary(p, BoundaryAction::Neighbour, n)
        }

        for j in 1..n - 1 {
            for i in 1..n - 1 {
                u[addr(i, j, n)] -= 0.5 * (p[addr(i + 1, j, n)] - p[addr(i - 1, j, n)]) / h;
                v[addr(i, j, n)] -= 0.5 * (p[addr(i, j + 1, n)] - p[addr(i, j - 1, n)]) / h;
            }
        }

        Fluid::set_boundary(u, BoundaryAction::NegativeX, n);
        Fluid::set_boundary(v, BoundaryAction::NegativeY, n);
    }

    fn set_boundary(x: &mut [f32], action: BoundaryAction, n: u32) {
        let xm = match action {
            BoundaryAction::NegativeX => -1.0f32,
            _ => 1.0f32,
        };
        let ym = match action {
            BoundaryAction::NegativeY => -1.0f32,
            _ => 1.0f32,
        };

        for i in 1..n {
            x[addr(0, i, n)] = xm * x[addr(1, i, n)];
            x[addr(n - 1, i, n)] = xm * x[addr(n - 2, i, n)];
            x[addr(i, 0, n)] = ym * x[addr(i, 1, n)];
            x[addr(i, n - 1, n)] = ym * x[addr(i, n - 2, n)];
        }
        x[addr(0, 0, n)] = 0.5 * (x[addr(1, 0, n)] + x[addr(0, 1, n)]);
        x[addr(n - 1, 0, n)] = 0.5 * (x[addr(n - 2, 0, n)] + x[addr(n - 1, 1, n)]);
        x[addr(0, n - 1, n)] = 0.5 * (x[addr(0, n - 2, n)] + x[addr(1, n - 1, n)]);
        x[addr(n - 1, n - 1, n)] = 0.5 * (x[addr(n - 1, n - 2, n)] + x[addr(n - 2, n - 1, n)]);
    }

    fn density_tick(
        x0: &mut Vec<f32>,
        x: &mut Vec<f32>,
        u: &Vec<f32>,
        v: &Vec<f32>,
        dt: f32,
        iter: u32,
        n: u32,
    ) {
        add_array(x, x0);
        std::mem::swap(x0, x);
        Fluid::diffuse(x, x0, dt, iter, n);
        std::mem::swap(x0, x);
        Fluid::advect(x, x0, u, v, dt, n);
    }

    fn velocity_tick(
        u0: &mut Vec<f32>,
        v0: &mut Vec<f32>,
        u: &mut Vec<f32>,
        v: &mut Vec<f32>,
        dt: f32,
        iter: u32,
        n: u32,
    ) {
        add_array(u, u0);
        add_array(v, v0);
        Fluid::diffuse(u0, u, dt, iter, n);
        Fluid::diffuse(v0, v, dt, iter, n);
        Fluid::project(u, v, u0, v0, iter, n);
        std::mem::swap(u0, u);
        std::mem::swap(v0, v);
        Fluid::advect(u, u0, &u0.clone(), v0, dt, n);
        Fluid::advect(v, v0, u0, &v0.clone(), dt, n);
        Fluid::project(u, v, u0, v0, iter, n);
    }

    fn reset_arrays(&mut self) {
        let area: usize = self.resolution.pow(2) as usize;
        self.d0 = vec![0.0f32; area];
        self.u0 = vec![0.0f32; area];
        self.v0 = vec![0.0f32; area];
    }

    pub fn tick(&mut self) {
        Fluid::velocity_tick(
            &mut self.u0,
            &mut self.v0,
            &mut self.u,
            &mut self.v,
            self.dt,
            self.iterations,
            self.resolution,
        );
        Fluid::density_tick(
            &mut self.d0,
            &mut self.d,
            &self.u,
            &self.v,
            self.dt,
            self.iterations,
            self.resolution,
        );

        self.reset_arrays();
    }
}
