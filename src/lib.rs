mod domain;
mod fractals;
mod pixel;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;
use wasm_bindgen::prelude::*;

pub use crate::domain::{Domain, Point};
pub use crate::fractals::{Fractal, Options, Variant};
pub use crate::pixel::Pixel;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const POOL_SIZE: usize = 8;

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct Frustal {
    width: usize,
    height: usize,
    data: Arc<Mutex<Vec<Pixel>>>,
    domain: Domain,
    fractal: Arc<dyn Fractal>,
    pool: ThreadPool,
}

#[wasm_bindgen]
impl Frustal {
    pub fn new(width: usize, height: usize) -> Frustal {
        set_panic_hook();
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
            width,
            height,
            data: Arc::new(Mutex::new(
                (0..width * height).map(|_| Pixel::void()).collect(),
            )),
            domain: Domain {
                min: Point { x: -2., y: -1.5 },
                max: Point { x: 2., y: 1.5 },
                width,
                height,
            },
            fractal: Variant::new(options),
            pool: ThreadPool::new(POOL_SIZE),
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.domain.width = width;
        self.domain.height = height;
        self.data = Arc::new(Mutex::new(
            (0..width * height).map(|_| Pixel::void()).collect(),
        ))
    }

    pub fn render(&mut self) {
        for (i, point) in self.domain.iter().enumerate() {
            let fractal = Arc::clone(&self.fractal);
            let data = Arc::clone(&self.data);
            self.pool.execute(move || {
                let pixel = fractal.get_pixel_at_point(point);
                match data.lock() {
                    Ok(mut array) => {
                        array[i].from(pixel);
                    }
                    Err(error) => panic!("Unlocking problem in thread: {:?}", error),
                };
            })
        }
        self.pool.join();
    }

    pub fn data_ptr(&self) -> *const Pixel {
        match self.data.lock() {
            Ok(array) => array.as_ptr(),
            Err(error) => panic!("Unlocking problem in getting ptr: {:?}", error),
        }
    }

    pub fn sync_options(&mut self, options: &Options) {
        if self.current_options().variant != options.variant {
            self.fractal = Variant::new(*options);
        } else {
            // self.fractal.set_options(*options);
        }
    }

    pub fn current_options(&self) -> Options {
        *self.fractal.options()
    }
}
