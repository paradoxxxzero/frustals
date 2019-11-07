use color_processing::Color;
use wasm_bindgen::prelude::*;

use crate::pixel::Pixel;
use crate::point::Point;
use num_complex::Complex64 as Complex;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub enum Variant {
    Mandelbrot,
    Julia,
    Mandelbar,
    BurningShip,
    Newton,
    Newton2,
    Newton3,
    Newton4,
    Newton5,
}

impl Variant {
    pub fn get_fractal(&self) -> Box<dyn Fractal> {
        match *self {
            // zn+1 = zn^d + c
            Variant::Mandelbrot => Box::new(Mandelbrot {}),
            // zn+1 = zn^d + c
            Variant::Julia => Box::new(Julia {}),
            // zn+1 = conj(zn)^d + c
            Variant::Mandelbar => Box::new(Mandelbar {}),
            // zn+1 = (abs(Re(zn)) + abs(Im(zn)))^d + c
            Variant::BurningShip => Box::new(BurningShip {}),

            // zn+1 = zn - p(zn) / p'(zn)
            // p = z³ - 1
            Variant::Newton => Box::new(Newton {
                polynomial: |z| z.powi(3) - Complex::new(1., 0.),
                derivative: |z| Complex::new(3., 0.) * z.powi(2),
                roots: vec![
                    (Complex::new(1., 0.), Channel::Red),
                    (Complex::new(-0.5, (3_f64).sqrt() / 2_f64), Channel::Green),
                    (Complex::new(-0.5, -(3_f64).sqrt() / 2_f64), Channel::Blue),
                ],
            }),
            // p = z³ - 2z + 2
            Variant::Newton2 => Box::new(Newton {
                polynomial: |z| z.powi(3) - Complex::new(2., 0.) * z + Complex::new(2., 0.),
                derivative: |z| Complex::new(3., 0.) * z.powi(2) - Complex::new(4., 0.),
                roots: vec![
                    (Complex::new(-1.7693, 0.), Channel::Cyan),
                    (Complex::new(0.88465, -0.58974), Channel::Yellow),
                    (Complex::new(0.88465, 0.58974), Channel::Magenta),
                ],
            }),
            // p = z⁶ + z³ - 1
            Variant::Newton3 => Box::new(Newton {
                polynomial: |z| z.powi(6) + z.powi(3) - Complex::new(1., 0.),
                derivative: |z| Complex::new(6., 0.) * z.powi(5) + Complex::new(3., 0.) * z.powi(2),
                roots: vec![
                    (Complex::new(0.58699, 1.01670), Channel::Red),
                    (Complex::new(0.85180, 0.0), Channel::Yellow),
                    (Complex::new(0.58699, -1.01670), Channel::Green),
                    (Complex::new(-0.42590, -0.73768), Channel::Cyan),
                    (Complex::new(-1.1740, 0.0), Channel::Blue),
                    (Complex::new(-0.42590, 0.73768), Channel::Magenta),
                ],
            }),
            // p = z⁵ - 2
            Variant::Newton4 => Box::new(Newton {
                polynomial: |z| z.powi(5) - Complex::new(2., 0.),
                derivative: |z| Complex::new(5., 0.) * z.powi(4),
                roots: vec![
                    (Complex::new(-0.929316, -0.675188), Channel::Red),
                    (Complex::new(-0.929316, 0.675188), Channel::Green),
                    (Complex::new(0.354967, -1.09248), Channel::Cyan),
                    (Complex::new(0.354967, 1.09248), Channel::Blue),
                    (Complex::new(1.1487, 0.), Channel::Magenta),
                ],
            }),
            // p = z³ - 1 + 1/z
            Variant::Newton5 => Box::new(Newton {
                polynomial: |z| z.powi(3) - Complex::new(1., 0.) + 1. / z,
                derivative: |z| {
                    (Complex::new(3., 0.) * z.powi(4) - Complex::new(1., 0.)) / z.powi(2)
                },
                roots: vec![
                    (Complex::new(-0.72714, -0.93410), Channel::Red),
                    (Complex::new(-0.72714, 0.93410), Channel::Cyan),
                    (Complex::new(0.72714, -0.43001), Channel::Magenta),
                    (Complex::new(0.72714, 0.43001), Channel::Blue),
                ],
            }),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Channel {
    Red,
    Green,
    Blue,
    Cyan,
    Magenta,
    Yellow,
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
#[derive(Clone, Copy, PartialEq)]
pub enum Colorization {
    Relative,
    RelativeBnW,
    AbsoluteHSL,
    AbsoluteLogHSL,
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub struct Options {
    pub precision: usize,
    pub smooth: bool,
    pub variant: Variant, // for gui purpose
    pub order: i32,
    pub const_real: f64,
    pub const_imaginary: f64,
    pub lightness: f64,
    pub colorization: Colorization,
}

pub trait Fractal {
    fn get_iterations_at_point(&self, point: Point, options: &Options) -> IterationsMaybe;

    fn get_pixel_for_iteration(&self, iterations: IterationsMaybe, options: &Options) -> Pixel {
        if let Some(Iterations { n, channel }) = iterations {
            match options.colorization {
                Colorization::Relative => {
                    // Expanding normalized iterations on the rgb spectrum:
                    let sn = if let Channel::All = channel {
                        3. * 255. * n / (options.precision as f64) * options.lightness
                    } else {
                        255. * (1. - (n / (options.precision as f64))) * options.lightness
                    };
                    match channel {
                        Channel::Red => Pixel::from_f64(sn, 0., 0.),
                        Channel::Yellow => Pixel::from_f64(sn, sn, 0.),
                        Channel::Green => Pixel::from_f64(0., sn, 0.),
                        Channel::Cyan => Pixel::from_f64(0., sn, sn),
                        Channel::Blue => Pixel::from_f64(0., 0., sn),
                        Channel::Magenta => Pixel::from_f64(sn, 0., sn),
                        Channel::All => Pixel::from_f64(sn, sn - 255., sn - 2. * 255.),
                    }
                }
                Colorization::RelativeBnW => {
                    // Expanding normalized iterations on the rgb spectrum:
                    let channel_inc = 255. / 6.;
                    let sn = if let Channel::All = channel {
                        255. * n / (options.precision as f64) * options.lightness
                    } else {
                        channel_inc * (1. - (n / (options.precision as f64))) * options.lightness
                    };

                    let channel_delta = match channel {
                        Channel::Red => 0.,
                        Channel::Yellow => channel_inc,
                        Channel::Green => channel_inc * 2.,
                        Channel::Cyan => channel_inc * 3.,
                        Channel::Blue => channel_inc * 4.,
                        Channel::Magenta => channel_inc * 5.,
                        Channel::All => 0.,
                    };
                    Pixel::from_f64(sn + channel_delta, sn + channel_delta, sn + channel_delta)
                }
                Colorization::AbsoluteHSL => {
                    let initial_hue = match channel {
                        Channel::Red => 0.,
                        Channel::Yellow => 60.,
                        Channel::Green => 120.,
                        Channel::Cyan => 180.,
                        Channel::Blue => 240.,
                        Channel::Magenta => 300.,
                        Channel::All => 0.,
                    };
                    if n > (options.lightness * 10.) {
                        Pixel::from_color(Color::new_hsl(
                            initial_hue + (n - (options.lightness * 10.)),
                            1.0,
                            0.5,
                        ))
                    } else {
                        Pixel::from_color(Color::new_hsl(
                            initial_hue,
                            1.0,
                            0.5 * n / (options.lightness * 10.),
                        ))
                    }
                }
                Colorization::AbsoluteLogHSL => {
                    let initial_hue = match channel {
                        Channel::Red => 0.,
                        Channel::Yellow => 60.,
                        Channel::Green => 120.,
                        Channel::Cyan => 180.,
                        Channel::Blue => 240.,
                        Channel::Magenta => 300.,
                        Channel::All => 0.,
                    };
                    if n > (options.lightness * 10.) {
                        Pixel::from_color(Color::new_hsl(
                            initial_hue + (1. + n - (options.lightness * 10.)).ln() * 10.,
                            1.0,
                            0.5,
                        ))
                    } else {
                        Pixel::from_color(Color::new_hsl(
                            initial_hue,
                            1.0,
                            0.5 * n / (options.lightness * 10.),
                        ))
                    }
                }
            }
        } else {
            Pixel::black()
        }
    }

    fn get_pixel_at_point(&self, point: Point, options: &Options) -> Pixel {
        self.get_pixel_for_iteration(self.get_iterations_at_point(point, options), options)
    }
}

pub struct Mandelbrot {}
impl Fractal for Mandelbrot {
    fn get_iterations_at_point(&self, point: Point, options: &Options) -> IterationsMaybe {
        let mut z = Complex::new(0_f64, 0_f64);
        let c = Complex::new(point.x, point.y);

        if options.order == 2 {
            let p = ((point.x - 1. / 4.).powi(2) + point.y.powi(2)).sqrt();
            if (point.x < p - 2. * p.powi(2) + 1. / 4.)
                || ((point.x + 1.).powi(2) + point.y.powi(2) < 1. / 16.)
            {
                return None;
            }
        }

        let mut iterations = 0;
        while iterations < options.precision {
            // zn+1 = zn^d + c
            if options.order == 2 {
                z = z * z + c;
            } else {
                z = z.powi(options.order) + c;
            }
            // |z| = sqrt(a² + b²)
            // |z|² = a² + b² =
            let mod2 = z.norm_sqr();
            // |z| > 2 => |z|² > 4
            if mod2 > 4. {
                let mut n = iterations as f64;
                if options.smooth {
                    // Smoothing is:
                    // ln( ln |zn| / ln B ) / ln d
                    // where B is max(|c|;2^(1/d-1)) and d is the order
                    n -= ((mod2.ln() / 2.)
                        / c.norm()
                            .max((2.0f64).powf(1. / (options.order as f64 - 1.))))
                    .ln()
                        / (options.order as f64).ln();
                }
                return Some(Iterations::all(n));
            }

            iterations += 1;
        }
        None
    }
}

pub struct Newton {
    polynomial: fn(Complex) -> Complex,
    derivative: fn(Complex) -> Complex,
    roots: Vec<(Complex, Channel)>,
}

impl Fractal for Newton {
    fn get_iterations_at_point(&self, point: Point, options: &Options) -> IterationsMaybe {
        let mut z = Complex::new(point.x, point.y);
        let c = Complex::new(options.const_real, options.const_imaginary);

        let mut iterations = 0;
        let epsilon = 0.00001_f64;

        let mut last_z;
        while iterations < options.precision {
            last_z = z;
            z -= c * (self.polynomial)(z) / (self.derivative)(z);
            for (root, channel) in self.roots.iter() {
                let convergence = (z - root).norm_sqr();
                if convergence < epsilon {
                    let mut n = iterations as f64;
                    if options.smooth {
                        let prev_ln_convergence = (last_z - root).norm_sqr().ln();
                        n += (epsilon.ln() - prev_ln_convergence)
                            / (convergence.ln() - prev_ln_convergence);
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
}

pub struct Julia {}
impl Fractal for Julia {
    fn get_iterations_at_point(&self, point: Point, options: &Options) -> IterationsMaybe {
        let mut z = Complex::new(point.x, point.y);
        let c = Complex::new(options.const_real, options.const_imaginary);
        let mut iterations = 0;

        while iterations < options.precision {
            // zn+1 = zn² + c
            z = z.powi(options.order) + c;
            let mod2 = z.norm_sqr();
            if mod2 > 4. {
                let mut n = iterations as f64;
                if options.smooth {
                    n -= mod2.ln().ln() * 1.25;
                }

                return Some(Iterations::all(n));
            }
            iterations += 1;
        }
        None
    }
}

pub struct Mandelbar {}
impl Fractal for Mandelbar {
    fn get_iterations_at_point(&self, point: Point, options: &Options) -> IterationsMaybe {
        let mut z = Complex::new(0_f64, 0_f64);
        let c = Complex::new(point.x, point.y);

        let mut iterations = 0;
        while iterations < options.precision {
            // zn+1 = conj(zn)^d + c
            z = z.conj().powi(options.order) + c;

            // |z| = sqrt(a² + b²)
            // |z|² = a² + b² =
            let mod2 = z.norm_sqr();
            // |z| > 2 => |z|² > 4
            if mod2 > 4. {
                let mut n = iterations as f64;
                if options.smooth {
                    // Smoothing is:
                    // ln( ln |zn| / ln B ) / ln d
                    // where B is max(|c|;2^(1/d-1)) and d is the order
                    n -= ((mod2.ln() / 2.)
                        / c.norm()
                            .max((2.0f64).powf(1. / (options.order as f64 - 1.))))
                    .ln()
                        / (options.order as f64).ln();
                }
                return Some(Iterations::all(n));
            }

            iterations += 1;
        }
        None
    }
}

pub struct BurningShip {}
impl Fractal for BurningShip {
    fn get_iterations_at_point(&self, point: Point, options: &Options) -> IterationsMaybe {
        let mut z = Complex::new(0_f64, 0_f64);
        let c = Complex::new(point.x, point.y);

        let mut iterations = 0;
        while iterations < options.precision {
            // zn+1 = (abs(Re(zn)) + abs(Im(zn)))² + c
            // We cheat by inverting z.im.abs() to make it upright
            z = Complex::new(z.re.abs(), z.im.abs()).powi(options.order) + c;

            // |z| = sqrt(a² + b²)
            // |z|² = a² + b²
            let mod2 = z.norm_sqr();
            // |z| > 2 => |z|² > 4
            if mod2 > 4. {
                let mut n = iterations as f64;
                if options.smooth {
                    // Smoothing is:
                    // ln( ln |zn| / ln B ) / ln d
                    // where B is max(|c|;2^(1/d-1)) and d is the order
                    n -= ((mod2.ln() / 2.)
                        / c.norm()
                            .max((2.0f64).powf(1. / (options.order as f64 - 1.))))
                    .ln()
                        / (options.order as f64).ln();
                }
                return Some(Iterations::all(n));
            }

            iterations += 1;
        }
        None
    }
}
