mod domain;
mod fractals;
mod pixel;

use crate::domain::{Domain, Point};
use crate::fractals::{Fractal, Mandelbrot, Newton, Options};
use crate::pixel::Pixel;
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub enum Preset {
    Mandelbrot,
    Mandelbrot3,
    Mandelbrot4,
    Mandelbrot5,
    Newton,
}

fn get_from_preset(preset: Preset) -> Box<dyn Fractal> {
    match preset {
        Preset::Mandelbrot => Box::new(Mandelbrot::new(
            Options {
                precision: 25.,
                smooth: true,
            },
            2,
        )),
        Preset::Mandelbrot3 => Box::new(Mandelbrot::new(
            Options {
                precision: 25.,
                smooth: true,
            },
            3,
        )),
        Preset::Mandelbrot4 => Box::new(Mandelbrot::new(
            Options {
                precision: 25.,
                smooth: true,
            },
            4,
        )),
        Preset::Mandelbrot5 => Box::new(Mandelbrot::new(
            Options {
                precision: 25.,
                smooth: true,
            },
            5,
        )),
        Preset::Newton => Box::new(Newton::new(Options {
            precision: 20.,
            smooth: true,
        })),
    }
}

#[wasm_bindgen]
pub struct Frustal {
    width: usize,
    height: usize,
    data: Vec<Pixel>,
    domain: Domain,
    fractal: Box<dyn Fractal>,
}

#[wasm_bindgen]
impl Frustal {
    pub fn new(width: usize, height: usize) -> Frustal {
        Frustal {
            width,
            height,
            data: (0..width * height).map(|_| Pixel::void()).collect(),
            domain: Domain {
                min: Point { x: -2., y: -1.5 },
                max: Point { x: 2., y: 1.5 },
                width,
                height,
            },
            fractal: get_from_preset(Preset::Mandelbrot),
        }
    }

    pub fn set_from_preset(&mut self, preset: Preset) {
        self.fractal = get_from_preset(preset);
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.domain.width = width;
        self.domain.height = height;
        self.data = (0..width * height).map(|_| Pixel::void()).collect()
    }

    pub fn render(&mut self) -> *const Pixel {
        set_panic_hook();

        for (i, point) in self.domain.iter().enumerate() {
            let pixel = self.fractal.get_pixel_at_point(point);
            self.data[i].from(pixel);
        }

        self.data.as_ptr()
    }
}
