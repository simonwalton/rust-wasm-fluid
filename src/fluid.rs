
use std::u32;

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

#[wasm_bindgen]
impl Fluid {
    #[inline(always)]
    fn addr(&self, x: u32, y: u32) -> u32 {
        x + (y * self.width)
    }

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
        let mut d0 = [0.0f32; AREA];
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

    pub fn tick(&mut self) {
        let a = &mut self.d;
        let b = &mut self.su;
        add_array(a, b);
    }
}