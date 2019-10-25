mod fractals;
mod pixel;
use crate::fractals::{Fractal, Mandelbrot, Newton, Options};
use crate::pixel::{Pixel, Point};
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub enum FractalType {
    Mandelbrot,
    Newton,
}

#[wasm_bindgen]
pub struct Frustal {
    width: usize,
    height: usize,
    data: Vec<Pixel>,
    fractal: Box<dyn Fractal>,
}

#[wasm_bindgen]
impl Frustal {
    pub fn new(width: usize, height: usize) -> Frustal {
        Frustal {
            width,
            height,
            data: (0..width * height).map(|_| Pixel::void()).collect(),
            fractal: Box::new(Newton::new(Options {
                precision: 20.,
                smooth: true,
            })),
        }
    }

    pub fn set_type(&mut self, fractal_type: FractalType) {
        let smooth = true;
        self.fractal = match fractal_type {
            FractalType::Mandelbrot => Box::new(Mandelbrot::new(Options {
                precision: 25.,
                smooth,
            })),
            FractalType::Newton => Box::new(Newton::new(Options {
                precision: 20.,
                smooth,
            })),
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.data = (0..width * height).map(|_| Pixel::void()).collect()
    }

    pub fn render(&mut self) -> *const Pixel {
        set_panic_hook();
        let x_min = -2.0;
        let x_max = 2.0;
        let y_min = -1.5;
        let y_max = 1.5;
        for x in 0..self.width {
            for y in 0..self.height {
                let i = x + (self.height - y - 1) * self.width;
                let point = Point {
                    x: x_min + (x as f64) * (x_max - x_min) / (self.width as f64),
                    y: y_min + (y as f64) * (y_max - y_min) / (self.height as f64),
                };
                let pixel = self.fractal.get_pixel_at_point(point);
                self.data[i].from(pixel);
            }
        }

        self.data.as_ptr()
    }
}
