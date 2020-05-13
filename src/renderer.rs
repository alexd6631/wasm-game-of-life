use crate::{Universe};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, ImageData, window};
use wasm_bindgen::{JsCast, Clamped};
use wasm_bindgen::__rt::core::cell::{Cell, RefCell};

#[wasm_bindgen]
pub struct Renderer {
    universe: Universe,
    context: CanvasRenderingContext2d,
    image_data: Vec<u8>
}


#[wasm_bindgen]
impl Renderer {
    pub fn new(universe: Universe, canvas: HtmlCanvasElement) -> Self {
        let context: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().dyn_into().unwrap();
        let image_data = vec![0u8; (universe.width * universe.height * 4) as usize];

        Renderer {
            universe,
            context,
            image_data
        }
    }

    pub fn draw(&mut self) {
        self.universe.tick();
        let vec = &mut self.image_data;
        for (i, cell) in self.universe.cells.iter().enumerate() {
            let is_alive = cell.is_alive();
            vec[i * 4] = if is_alive { 80 } else { 0 };
            vec[i * 4 + 1] = if is_alive { 80 } else { 0 };
            vec[i * 4 + 2] = if is_alive { 200 } else { 0 };
            vec[i * 4 + 3] = 255;
        }
        let image_data = ImageData::new_with_u8_clamped_array(
            Clamped(vec),
            self.universe.width
        ).unwrap();
        self.context.put_image_data(&image_data, 0.0, 0.0).unwrap();
    }
}

