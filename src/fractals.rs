use wasm_bindgen::prelude::*;

use crate::domain::Point;
use crate::pixel::Pixel;
use num_complex::Complex;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub enum Variant {
    Mandelbrot,
    Newton,
}

impl Variant {
    pub fn new(options: Options) -> Box<dyn Fractal> {
        match options.variant {
            Variant::Mandelbrot => Box::new(Mandelbrot::new(options)),
            Variant::Newton => Box::new(Newton::new(options)),
        }
    }
}

enum Channel {
    Red,
    Green,
    Blue,
    All,
}

pub struct Iterations {
    n: f64,
    channel: Channel,
}

impl Iterations {
    fn all(n: f64) -> Iterations {
        Iterations {
            n,
            channel: Channel::All,
        }
    }
}

pub type IterationsMaybe = Option<Iterations>;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Options {
    pub precision: f64,
    pub smooth: bool,
    pub order: i32,
    pub variant: Variant, // for gui purpose
}

#[wasm_bindgen]
impl Options {
    pub fn dup(&self) -> Options {
        self.clone()
    }
}

pub trait Fractal {
    fn options(&self) -> &Options;
    fn set_options(&mut self, options: Options);

    fn get_iterations_at_point(&self, point: Point) -> IterationsMaybe;

    fn get_pixel_for_iteration(&self, iterations: IterationsMaybe) -> Pixel {
        if let Some(Iterations { n, channel }) = iterations {
            if let Channel::All = channel {
                // Expanding normalized iterations on the rgb spectrum:
                let order = 3. * 255. * n / self.options().precision;
                Pixel::from_f64(order, order - 255., order - 2. * 255.)
            } else {
                panic!(
                    "Default implementation for get_pixel_for_iteration only support channel ALL"
                );
            }
        } else {
            Pixel::void()
        }
    }

    fn get_pixel_at_point(&self, point: Point) -> Pixel {
        self.get_pixel_for_iteration(self.get_iterations_at_point(point))
    }
}

pub struct Mandelbrot {
    options: Options,
}

impl Mandelbrot {
    pub fn new(options: Options) -> Mandelbrot {
        Mandelbrot { options }
    }
}

impl Fractal for Mandelbrot {
    fn options(&self) -> &Options {
        &self.options
    }

    fn set_options(&mut self, options: Options) {
        self.options = options
    }

    fn get_iterations_at_point(&self, point: Point) -> IterationsMaybe {
        let mut z = Complex::new(0_f64, 0_f64);
        let c = Complex::new(point.x, point.y);

        let mut n = 0_f64;
        while n < self.options.precision {
            // |z| = sqrt(a^2 + b^2)
            // |z|^2 = a^2 + b^2 =
            let mod2 = z.norm_sqr();
            // |z| > 2 => |z|^2 > 4
            if mod2 > 4. {
                if self.options.smooth {
                    // Smoothing is:
                    // ln( ln |zn| / ln B ) / ln d
                    // where B is max(|c|;2^(1/d-1)) and d is the order
                    n -= ((mod2.ln() / 2.)
                        / c.norm()
                            .max((2.0f64).powf(1. / (self.options.order as f64 - 1.))))
                    .ln()
                        / (self.options.order as f64).ln();
                }
                return Some(Iterations::all(n));
            }

            // zn+1 = zn^d + c
            z = z.powi(self.options.order) + c;
            n += 1.;
        }
        None
    }
}

pub struct Newton {
    options: Options,
}

impl Newton {
    pub fn new(options: Options) -> Newton {
        Newton { options }
    }
}

impl Fractal for Newton {
    fn options(&self) -> &Options {
        &self.options
    }

    fn set_options(&mut self, options: Options) {
        self.options = options
    }

    fn get_iterations_at_point(&self, point: Point) -> IterationsMaybe {
        let mut r = point.x;
        let mut i = point.y;
        let mut n = 0.0_f64;
        while n < self.options.precision {
            let rr = r * r;
            let ii = i * i;

            let r2 = rr - ii;
            let i2 = 2. * r * i;

            let r3 = r * r2 - i * i2;
            let i3 = r * i2 + i * r2;

            let d = (r2 * r2 + i2 * i2) * 3.;
            r -= (r2 * (r3 - 1.) + i2 * i3) / d;
            i -= (r2 * i3 - i2 * (r3 - 1.)) / d;

            let n1 = (r - 1.) * (r - 1.0) + i * i;
            let n2 = (r + 0.5) * (r + 0.5) + (i - 0.866025404) * (i - 0.866025404);
            let n3 = (r + 0.5) * (r + 0.5) + (i + 0.866025404) * (i + 0.866025404);

            if n1 < 0.0001 || n2 < 0.0001 || n3 < 0.0001 {
                let channel = if n1 < 0.0001 {
                    Channel::Red
                } else if n2 < 0.0001 {
                    Channel::Green
                } else {
                    Channel::Blue
                };

                if self.options.smooth {
                    let current_n = match channel {
                        Channel::Red => n1,
                        Channel::Green => n2,
                        Channel::Blue => n3,
                        _ => 0.,
                    };
                    n -= (1. / (50. * current_n)).ln().ln();
                }
                return Some(Iterations { n, channel });
            }
            n += 1.0;
        }
        None
    }

    fn get_pixel_for_iteration(&self, iterations: IterationsMaybe) -> Pixel {
        if let Some(iterations) = iterations {
            // Expanding normalized iterations on the rgb spectrum:
            let sn = 255. - iterations.n * self.options.precision;
            match iterations.channel {
                Channel::Red => Pixel::from_f64(sn, 0., 0.),
                Channel::Green => Pixel::from_f64(0., sn, 0.),
                Channel::Blue => Pixel::from_f64(0., 0., sn),
                _ => Pixel::void(),
            }
        } else {
            Pixel::void()
        }
    }
}
