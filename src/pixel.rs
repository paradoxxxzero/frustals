use color_processing::Color;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Pixel {
        Pixel { r, g, b, a }
    }

    pub fn from_f64(r: f64, g: f64, b: f64) -> Pixel {
        Pixel {
            r: 0f64.max(r).min(255f64).round() as u8,
            g: 0f64.max(g).min(255f64).round() as u8,
            b: 0f64.max(b).min(255f64).round() as u8,
            a: 255,
        }
    }

    pub fn from_color(color: Color) -> Pixel {
        Pixel {
            r: color.red,
            g: color.green,
            b: color.blue,
            a: 255,
        }
    }

    pub fn black() -> Pixel {
        Pixel::new(0, 0, 0, 255)
    }

    pub fn void() -> Pixel {
        Pixel::new(0, 0, 0, 0)
    }

    pub fn from(&mut self, source: Pixel) {
        self.r = source.r;
        self.g = source.g;
        self.b = source.b;
        self.a = source.a;
    }
}
