mod domain;
mod fractals;
mod pixel;
use wasm_bindgen::prelude::*;

pub use crate::domain::{Domain, Point};
pub use crate::fractals::{Fractal, Options, Variant};
pub use crate::pixel::Pixel;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
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
        let options = Options {
            variant: Variant::Mandelbrot,
            smooth: true,
            precision: 25,
            order: 2,
            julia_real: -0.8,
            julia_imaginary: 0.156,
        };
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
            fractal: Variant::new(options),
        }
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

    pub fn sync_options(&mut self, options: &Options) {
        if self.current_options().variant != options.variant {
            self.fractal = Variant::new(*options);
        } else {
            self.fractal.set_options(*options);
        }
    }

    pub fn current_options(&self) -> Options {
        *self.fractal.options()
    }
}
