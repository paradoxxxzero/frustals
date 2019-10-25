use crate::pixel::{Pixel, Point};

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

pub struct Options {
    pub precision: f64,
    pub smooth: bool,
}

pub trait Fractal {
    fn new(options: Options) -> Self
    where
        Self: Sized;

    fn options(&self) -> &Options;

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

impl Fractal for Mandelbrot {
    fn new(options: Options) -> Mandelbrot {
        Mandelbrot { options }
    }

    fn options(&self) -> &Options {
        &self.options
    }

    fn get_iterations_at_point(&self, point: Point) -> IterationsMaybe {
        let mut a = 0_f64;
        let mut b = 0_f64;

        let mut n = 0_f64;
        while n < self.options.precision {
            let a2 = a * a;
            let b2 = b * b;
            // |z| = sqrt(a^2 + b^2)
            // |z|^2 = a^2 + b^2 =
            let mod2 = a2 + b2;
            // |z| > 2 => |z|^2 > 4
            if mod2 > 4. {
                if self.options.smooth {
                    n += 1. - (mod2.log2() / 2.).log2()
                }
                return Some(Iterations::all(n));
            }

            // zn+1 = zn^2 + c
            // an+1 + ibn+1 = an^2 - bn^2 + i2anbn + x + iy
            // an+1 = an^2 - bn^2 + x
            // bn+1 = 2anbn + y
            n += 1.;
            b = 2. * a * b + point.y;
            a = a2 - b2 + point.x;
        }
        None
    }
}

pub struct Newton {
    options: Options,
}

impl Fractal for Newton {
    fn new(options: Options) -> Newton {
        Newton { options }
    }

    fn options(&self) -> &Options {
        &self.options
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
