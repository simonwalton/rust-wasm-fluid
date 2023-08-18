
use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;

#[wasm_bindgen]
pub struct Fluid {
    width: u32,
    height: u32,
    cells: Vec<u8>
}


#[wasm_bindgen]
impl Fluid {
    pub fn new() -> Fluid {
        let width = 800;
        let height = 800;
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 {
                    0
                }
                else {
                    255
                }
            })
            .collect();
        Fluid {
            width,
            height,
            cells
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> Uint8Array {
        unsafe { Uint8Array::view(&self.cells.as_slice()) }
    }

    pub fn draw() {

    }
}