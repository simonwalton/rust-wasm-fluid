
use std::u32;
use std::cmp;

use wasm_bindgen::prelude::*;
use js_sys::Float32Array;

const WIDTH: u32 = 150;
const HEIGHT: u32 = 150;
const AREA: usize = (WIDTH * HEIGHT) as usize;

#[wasm_bindgen]
pub struct Fluid {
    width: u32,
    height: u32,
    dt: f32,
    u: Vec<f32>,
    u0: Vec<f32>,
    v: Vec<f32>,
    v0:Vec<f32>,
    d: Vec<f32>,
    d0: Vec<f32>,
}

fn add_array(a: &mut Vec<f32>, b: &Vec<f32>) {
    for i in 0..a.len() {
        a[i] += b[i];
    }
}

#[inline(always)]
fn addr(x: u32, y: u32) -> usize {
    (x + (y * WIDTH)) as usize
}

#[inline(always)]
fn clamp(x: f32) -> f32 {
    x.max(0.0f32).min(1.0f32)
}

#[inline(always)]
fn clampAB(x: f32, a: f32, b: f32) -> f32 {
    x.max(a).min(b)
}

#[inline(always)]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + ((b - a) * t)
}

#[wasm_bindgen]
impl Fluid {
    pub fn new() -> Fluid {
        let width = WIDTH;
        let height = HEIGHT;
        let dt = 0.001f32;
        let mut d0: Vec<f32> = vec![0.0f32; AREA];
        let mut u = vec![0.0f32; AREA];
        let mut u0 = vec![0.0f32; AREA];
        let mut v = vec![0.0f32; AREA];
        let mut v0 = vec![0.0f32; AREA];
        let mut d = vec![0.0f32; AREA];
            
        Fluid {
            width,
            height,
            dt,
            u,
            u0,
            v,
            v0,
            d,
            d0
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_dt(&mut self, dt: f32) {
        self.dt = dt;
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

    fn diffuse(x: &mut Vec<f32>, x0: &Vec<f32>, dt: f32) {
        let diff = 0.005f32;
        let a = dt * diff * (WIDTH * HEIGHT) as f32;

        for k in 0..10 {
            for j in 1..HEIGHT-1 {
                for i in 1..WIDTH-1 {
                    let neighbours = x[addr(i-1,j)] + x[addr(i+1,j)] + x[addr(i,j-1)] + x[addr(i,j+1)];
                    x[addr(i,j)] = clamp((x0[addr(i,j)] + a * neighbours) / (1.0f32+4.0f32*a));
                }
            }
            Fluid::set_boundary(x)
        }
    }

    fn advect(d: &mut Vec<f32>, d0: &Vec<f32>, u: &Vec<f32>, v: &Vec<f32>) {
        let dt = 0.003f32;
        let dt0 = dt * WIDTH as f32;

        for j in 1..HEIGHT-1 {
            for i in 1..WIDTH-1 { 
                let xp = clampAB(i as f32 - dt0 * u[addr(i,j)], 1.5f32, WIDTH as f32 - 2.5f32);
                let yp = clampAB(j as f32 - dt0 * v[addr(i,j)], 1.5f32, HEIGHT as f32 - 2.5f32);
                let x0 = xp.floor() as u32; let x1 = x0 + 1;
                let y0 = yp.floor() as u32; let y1 = y0 + 1;

                d[addr(i,j)] = lerp(
                    lerp(d0[addr(x0, y0)], d0[addr(x0, y1)], yp - y0 as f32),
                    lerp(d0[addr(x1, y0)], d0[addr(x1, y1)], yp - y0 as f32),
                    xp - x0 as f32
                )
            }
        }

        Fluid::set_boundary(d)
    }

    fn project(u: &mut Vec<f32>, v: &mut Vec<f32>, p: &mut Vec<f32>, d: &mut Vec<f32>) {
        let h = 1.0f32 / WIDTH as f32;

        for j in 1..HEIGHT-1 {
            for i in 1..WIDTH-1 {  
                d[addr(i,j)] = -0.5 * h * (u[addr(i+1,j)]-u[addr(i-1,j)]+v[addr(i,j+1)]-v[addr(i,j-1)]);
                p[addr(i,j)] = 0.0f32;
            }
        }
        Fluid::set_boundary(d);
        Fluid::set_boundary(p);

        for k in 0..10 {
            for j in 1..HEIGHT-1 {
                for i in 1..WIDTH-1 {  
                    p[addr(i,j)] = (d[addr(i,j)]+p[addr(i-1,j)]+p[addr(i+1,j)]+p[addr(i,j-1)]+p[addr(i,j+1)])/4.0f32;
                }
            }
            Fluid::set_boundary(p)
        }

        for j in 1..HEIGHT-1 {
            for i in 1..WIDTH-1 {  
                u[addr(i,j)] -= 0.5 * (p[addr(i+1,j)] - p[addr(i-1,j)]) / h;
                v[addr(i,j)] -= 0.5 * (p[addr(i,j+1)] - p[addr(i,j-1)]) / h;
            }
        } 

        Fluid::set_boundary(u);
        Fluid::set_boundary(v);
    }

    fn set_boundary(x: &mut Vec<f32>) {
        let N = WIDTH;
        for i in 1..N {
            x[addr(0, i)] = x[addr(1,i)];
            x[addr(N-1, i)] = x[addr(N-2,i)];
            x[addr(i, 0)] = x[addr(i, 1)];
            x[addr(i, N-1)] = x[addr(i,N-2)];
        }
        x[addr(0,0)] = 0.5 * (x[addr(1,0)] + x[addr(0,1)]);
        x[addr(N-1,0)] = 0.5 * (x[addr(N-2,0)] + x[addr(N-1,1)]);
        x[addr(0,N-1)] = 0.5 * (x[addr(0,N-2)] + x[addr(1,N-1)]);
        x[addr(N-1,N-1)] = 0.5 * (x[addr(N-1,N-2)] + x[addr(N-2,N-1)]);
    }

    fn density_tick(x0: &mut Vec<f32>, x: &mut Vec<f32>, u: &Vec<f32>, v: &Vec<f32>, dt: f32) {
        add_array(x, x0);
        std::mem::swap(x0, x);
        Fluid::diffuse(x, x0, dt);
        std::mem::swap(x0, x);
        Fluid::advect(x, x0, u, v);
    }

    fn velocity_tick(u0: &mut Vec<f32>, v0: &mut Vec<f32>, u: &mut Vec<f32>, v: &mut Vec<f32>, dt: f32) {
        add_array(u, u0);
        add_array(v, v0);
        Fluid::diffuse(u0, u, dt);
        Fluid::diffuse(v0, v, dt);
        Fluid::project(u, v, u0, v0);
        std::mem::swap(u0, u);
        std::mem::swap(v0, v);
        Fluid::advect(u, u0, &u0.clone(), v0);
        Fluid::advect(v, v0, u0, &v0.clone());
        Fluid::project(u, v, u0, v0);
    }

    pub fn tick(&mut self) {
        Fluid::velocity_tick(&mut self.u0, &mut self.v0, &mut self.u, &mut self.v, self.dt);
        Fluid::density_tick(&mut self.d0, &mut self.d, &mut self.u, &mut self.v, self.dt);

        self.d0 = vec![0.0f32; AREA];
        self.u0 = vec![0.0f32; AREA];
        self.v0 = vec![0.0f32; AREA];
    }
}