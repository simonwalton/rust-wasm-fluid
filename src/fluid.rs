
use std::u32;
use std::cmp;

use wasm_bindgen::prelude::*;
use js_sys::Float32Array;

const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;
const AREA: usize = (WIDTH * HEIGHT) as usize;

#[wasm_bindgen]
pub struct Fluid {
    width: u32,
    height: u32,
    u: [f32; AREA],
    u0: [f32; AREA],
    v: [f32; AREA],
    v0:[f32; AREA],
    d: [f32; AREA],
    d0: [f32; AREA],
}

fn add_array(a: &mut[f32; AREA], b: &mut [f32; AREA]) {
    for i in 0..AREA {
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
        let mut d: [f32; AREA] = [0.0f32; AREA];

        for j in 1..HEIGHT-2 {
            for i in 1..WIDTH-2 {
                d[addr(i,j)] = 1.0f32;
                if i > 40 && j > 40 && i < 60 && j < 60 {
                    d[addr(i,j)] = 1.0f32;
                }
                else {
                    d[addr(i,j)] = 0.0f32;
                }

            }
        }

        let mut u = [0.0f32; AREA];
        let mut u0 = [0.0f32; AREA];
        let mut v = [0.0f32; AREA];
        let mut v0 = [0.0f32; AREA];
        let mut d0 = d;//.copy();//[0.0f32; AREA];
            
        Fluid {
            width,
            height,
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

    pub fn density(&self) -> Float32Array {
        unsafe { Float32Array::view(&self.d) }
    }

    pub fn source_u(&self) -> Float32Array {
        unsafe { Float32Array::view(&self.u) }
    }

    pub fn source_v(&self) -> Float32Array {
        unsafe { Float32Array::view(&self.v) }
    }

    fn diffuse(x0: &mut[f32; AREA], x: &mut [f32; AREA]) {
        let dt = 0.001f32;
        let diff = 0.01f32;
        let a = dt * diff * (WIDTH * HEIGHT) as f32;

        for k in 0..3 {
            for j in 1..HEIGHT-2 {
                for i in 1..WIDTH-2 {
                    let neighbours = x[addr(i-1,j)] + x[addr(i+1,j)] + x[addr(i, j-1)] + x[addr(i,j+1)];
                    x[addr(i,j)] = clamp((x0[addr(i,j)] + a * neighbours) / (1.0f32+4.0f32*a));
                }
            }
        }
    }

    fn advect(d0: &mut[f32; AREA], d: &mut[f32; AREA], u: &mut[f32; AREA], v: &mut[f32; AREA]) {
        let dt = 0.01f32;
        let dt0 = dt * WIDTH as f32;

        for j in 1..HEIGHT-2 {
            for i in 1..WIDTH-2 { 
                let xp = clampAB(i as f32 - dt0 * u[addr(i,j)], 0.5f32, WIDTH as f32 - 1.5f32);
                let yp = clampAB(j as f32 - dt0 * v[addr(i,j)], 0.5f32, HEIGHT as f32 - 1.5f32);
                let x0 = xp.floor() as u32; let x1 = x0 + 1;
                let y0 = yp.floor() as u32; let y1 = y0 + 1;

                d[addr(i,j)] = clamp(lerp(
                    lerp(d0[addr(x0, y0)], d0[addr(x0, y1)], yp - y0 as f32),
                    lerp(d0[addr(x1, y0)], d0[addr(x1, y1)], yp - y0 as f32),
                    xp - x0 as f32
                ))
            }
        }
    }

    fn density_tick(x0: &mut[f32; AREA], x: &mut [f32; AREA], u: &mut[f32; AREA], v: &mut[f32; AREA]) {
        Fluid::diffuse(x0, x);
        std::mem::swap(x0, x);
        Fluid::advect(x0, x, u, v);
        std::mem::swap(x0, x);
    }

    fn velocity_step(&mut self) {

    }

    pub fn tick(&mut self) {
        //add_array(a, b);
        Fluid::density_tick(&mut self.d0, &mut self.d, &mut self.u, &mut self.v);
    }
}