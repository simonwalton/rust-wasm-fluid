use std::u32;
use js_sys::Float32Array;
use wasm_bindgen::prelude::*;

use crate::util::add_array;
use crate::field::{diffuse, project, advect};

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
    d0: Vec<f32>        // density field (alternate)
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
        diffuse(x, x0, dt, iter, n);
        std::mem::swap(x0, x);
        advect(x, x0, u, v, dt, n);
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
        diffuse(u0, u, dt, iter, n);
        diffuse(v0, v, dt, iter, n);
        project(u, v, u0, v0, iter, n);
        std::mem::swap(u0, u);
        std::mem::swap(v0, v);
        advect(u, u0, &u0.clone(), v0, dt, n);
        advect(v, v0, u0, &v0.clone(), dt, n);
        project(u, v, u0, v0, iter, n);
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

