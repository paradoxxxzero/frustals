mod domain;
mod fractals;
mod pixel;
mod point;
use wasm_bindgen::prelude::*;

pub use crate::domain::Domain;
pub use crate::fractals::{Fractal, Options, Variant};
pub use crate::pixel::Pixel;
pub use crate::point::Point;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct Frustal {
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
            lightness: 1.0,
        };
        Frustal {
            data: (0..width * height).map(|_| Pixel::void()).collect(),
            domain: Domain::new(Point::new(width as f64, height as f64)),
            fractal: Variant::new(options),
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.domain.resize(Point::new(width as f64, height as f64));
        self.data = (0..width * height).map(|_| Pixel::void()).collect()
    }

    pub fn shift_domain(&mut self, point: Point) {
        self.domain.shift(point)
    }

    pub fn scale_domain(&mut self, factor: f64, center: Point) {
        self.domain.scale(Point::new(factor, factor), center)
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
