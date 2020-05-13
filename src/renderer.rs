use crate::{Universe};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d, ImageData, window};
use wasm_bindgen::{JsCast, Clamped};
use wasm_bindgen::__rt::core::cell::{Cell, RefCell};

#[wasm_bindgen]
pub struct Renderer {
    universe: Universe,
    context: CanvasRenderingContext2d,
    image_data: ImageBuffer
}

struct ImageBuffer(Vec<u8>);
struct Color(u8, u8, u8, u8);

impl ImageBuffer {
    fn set_color(&mut self, i: usize, color: &Color) {
        self.0[i * 4] = color.0;
        self.0[i * 4 + 1] = color.1;
        self.0[i * 4 + 2] = color.2;
        self.0[i * 4 + 3] = color.3;
    }
}

static CELL_COLOR: Color = Color(80, 80, 255, 255);
static BLACK: Color = Color(0, 0, 0, 250);

#[wasm_bindgen]
impl Renderer {
    pub fn new(universe: Universe, canvas: HtmlCanvasElement) -> Self {
        let context: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().dyn_into().unwrap();
        let image_data = ImageBuffer(vec![0u8; (universe.width * universe.height * 4) as usize]);

        Renderer {
            universe,
            context,
            image_data
        }
    }

    pub fn draw(&mut self) {
        self.universe.tick();
        for (i, cell) in self.universe.cells.iter().enumerate() {
            let color = if cell.is_alive() { &CELL_COLOR } else { &BLACK };
            self.image_data.set_color(i, color);
        }
        let image_data = ImageData::new_with_u8_clamped_array(
            Clamped(&mut self.image_data.0),
            self.universe.width
        ).unwrap();
        self.context.put_image_data(&image_data, 0.0, 0.0).unwrap();
    }
}

