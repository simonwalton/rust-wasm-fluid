
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
    su: [f32; AREA],
    sv: [f32; AREA],
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

#[wasm_bindgen]
impl Fluid {


    pub fn new() -> Fluid {
        let width = WIDTH;
        let height = HEIGHT;
        let mut d: [f32; AREA] = [0.0f32; AREA];

        (0..AREA)
            .for_each(|i| {
                if i % 2 == 0 {
                    d[i] = 0.0f32;
                }
                else {
                    d[i] = 1.0f32;
                }
            });

        let mut u = [0.0f32; AREA];
        let mut u0 = [0.0f32; AREA];
        let mut v = [0.0f32; AREA];
        let mut v0 = [0.0f32; AREA];
        let mut d0 = d;//.copy();//[0.0f32; AREA];
        let mut su = [0.0f32; AREA];
        let mut sv = [0.0f32; AREA];
            
        Fluid {
            width,
            height,
            u,
            u0,
            v,
            v0,
            d,
            d0,
            su,
            sv
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
        unsafe { Float32Array::view(&self.su) }
    }

    pub fn source_v(&self) -> Float32Array {
        unsafe { Float32Array::view(&self.sv) }
    }

    fn diffuse(&mut self) {
        let dt = 0.01f32;
        let diff = 0.1f32;
        let a = dt * diff * (WIDTH * HEIGHT) as f32;
        for k in 0..3 {
            for y in 1..HEIGHT-2 {
                for x in 1..WIDTH-2 {
                    let neighbours = self.d[addr(x-1,y)] + self.d[addr(x+1,y)] + self.d[addr(x, y-1)] + self.d[addr(x, y+1)];
                    self.d[addr(x,y)] = clamp((self.d0[addr(x,y)] + a * neighbours) / (1.0f32+4.0f32*a));
                }
            }
        }
    }

    fn swap_arrays(&mut self) {
        let tmp = self.d0; self.d0 = self.d; self.d = tmp;
    }

    pub fn tick(&mut self) {
        let a = &mut self.d;
        let b = &mut self.su;
        add_array(a, b);
        self.swap_arrays();
        self.diffuse();
        //self.swap_arrays();
    }
}