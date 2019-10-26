use wasm_bindgen::prelude::*;

use crate::domain::Point;
use crate::pixel::Pixel;
use num_complex::Complex;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub enum Variant {
    Mandelbrot,
    Newton,
    Julia,
    Mandelbar,
    BurningShip,
}

impl Variant {
    pub fn new(options: Options) -> Box<dyn Fractal> {
        match options.variant {
            Variant::Mandelbrot => Box::new(Mandelbrot::new(options)),
            Variant::Newton => Box::new(Newton::new(options)),
            Variant::Julia => Box::new(Julia::new(options)),
            Variant::Mandelbar => Box::new(Mandelbar::new(options)),
            Variant::BurningShip => Box::new(BurningShip::new(options)),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
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
    pub precision: usize,
    pub smooth: bool,
    pub variant: Variant, // for gui purpose
    pub order: i32,
    pub julia_real: f64,
    pub julia_imaginary: f64,
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
                let order = 3. * 255. * n / (self.options().precision as f64);
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

        let mut iterations = 0;
        while iterations < self.options.precision {
            // zn+1 = zn^d + c
            z = z.powi(self.options.order) + c;

            // |z| = sqrt(a^2 + b^2)
            // |z|^2 = a^2 + b^2 =
            let mod2 = z.norm_sqr();
            // |z| > 2 => |z|^2 > 4
            if mod2 > 4. {
                let mut n = iterations as f64;
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

            iterations += 1;
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
        let mut z = Complex::new(point.x, point.y);

        let mut iterations = 0;
        let epsilon = 0.0001_f64;
        let roots = [
            (Complex::new(1., 0.), Channel::Red),
            (Complex::new(-0.5, (3_f64).sqrt() / 2_f64), Channel::Green),
            (Complex::new(-0.5, -(3_f64).sqrt() / 2_f64), Channel::Blue),
        ];

        while iterations < self.options.precision {
            z -= (z.powi(3) - Complex::new(1., 0.)) / (Complex::new(3., 0.) * z.powi(2));

            for (root, channel) in roots.iter() {
                let convergence = (z - root).norm_sqr();
                if convergence < epsilon {
                    let mut n = iterations as f64;
                    if self.options.smooth {
                        n -= (1. / (50. * convergence)).ln().ln()
                    }
                    return Some(Iterations {
                        n,
                        channel: *channel,
                    });
                }
            }
            iterations += 1;
        }
        None
    }

    fn get_pixel_for_iteration(&self, iterations: IterationsMaybe) -> Pixel {
        if let Some(iterations) = iterations {
            // Expanding normalized iterations on the rgb spectrum:
            let sn = 255. - iterations.n * (self.options.precision as f64);
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

pub struct Julia {
    options: Options,
}

impl Julia {
    pub fn new(options: Options) -> Julia {
        Julia { options }
    }
}

impl Fractal for Julia {
    fn options(&self) -> &Options {
        &self.options
    }

    fn set_options(&mut self, options: Options) {
        self.options = options
    }

    fn get_iterations_at_point(&self, point: Point) -> IterationsMaybe {
        let mut z = Complex::new(point.x, point.y);
        // let c = Complex::new(0.3, 0.5);
        let c = Complex::new(self.options.julia_real, self.options.julia_imaginary);
        let mut iterations = 0;

        while iterations < self.options.precision {
            // zn+1 = zn^2 + c
            z = z.powi(self.options.order) + c;
            let mod2 = z.norm_sqr();
            if mod2 > 4. {
                let mut n = iterations as f64;
                if self.options.smooth {
                    n -= mod2.ln().ln() * 1.25;
                }

                return Some(Iterations::all(n));
            }
            iterations += 1;
        }
        None
    }
}

pub struct Mandelbar {
    options: Options,
}

impl Mandelbar {
    pub fn new(options: Options) -> Mandelbar {
        Mandelbar { options }
    }
}

impl Fractal for Mandelbar {
    fn options(&self) -> &Options {
        &self.options
    }

    fn set_options(&mut self, options: Options) {
        self.options = options
    }

    fn get_iterations_at_point(&self, point: Point) -> IterationsMaybe {
        let mut z = Complex::new(0_f64, 0_f64);
        let c = Complex::new(point.x, point.y);

        let mut iterations = 0;
        while iterations < self.options.precision {
            // zn+1 = conj(zn)^d + c
            z = z.conj().powi(self.options.order) + c;

            // |z| = sqrt(a^2 + b^2)
            // |z|^2 = a^2 + b^2 =
            let mod2 = z.norm_sqr();
            // |z| > 2 => |z|^2 > 4
            if mod2 > 4. {
                let mut n = iterations as f64;
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

            iterations += 1;
        }
        None
    }
}

pub struct BurningShip {
    options: Options,
}

impl BurningShip {
    pub fn new(options: Options) -> BurningShip {
        BurningShip { options }
    }
}

impl Fractal for BurningShip {
    fn options(&self) -> &Options {
        &self.options
    }

    fn set_options(&mut self, options: Options) {
        self.options = options
    }

    fn get_iterations_at_point(&self, point: Point) -> IterationsMaybe {
        let mut z = Complex::new(0_f64, 0_f64);
        let c = Complex::new(point.x, point.y);

        let mut iterations = 0;
        while iterations < self.options.precision {
            // zn+1 = conj(zn)^d + c
            z = Complex::new(z.re.abs(), z.im.abs()).powi(self.options.order) + c;

            // |z| = sqrt(a^2 + b^2)
            // |z|^2 = a^2 + b^2 =
            let mod2 = z.norm_sqr();
            // |z| > 2 => |z|^2 > 4
            if mod2 > 4. {
                let mut n = iterations as f64;
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

            iterations += 1;
        }
        None
    }
}
