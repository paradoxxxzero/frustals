use crate::point::Point;

pub struct DomainIterator<'a> {
    x: usize,
    y: usize,
    domain: &'a Domain,
}

impl<'a> DomainIterator<'a> {
    fn new(domain: &'a Domain) -> DomainIterator {
        DomainIterator { x: 0, y: 0, domain }
    }
}

impl<'a> Iterator for DomainIterator<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        if self.x as f64 >= self.domain.size.x {
            self.x = 0;
            self.y += 1;
            if self.y as f64 >= self.domain.size.y {
                return None;
            }
        }
        self.x += 1;

        Some(
            self.domain
                .project(Point::new(self.x as f64, self.y as f64)),
        )
    }
}

pub struct Domain {
    pub min: Point,
    pub max: Point,
    pub size: Point, // contains canvas width and height
}

impl Domain {
    pub fn new(size: Point) -> Domain {
        let point = if size.x > size.y {
            Point::new(2. * size.x / size.y, 1.5)
        } else {
            Point::new(2., 1.5 * size.y / size.x)
        };
        Domain {
            min: point * Point::new(-1., -1.),
            max: point,
            size,
        }
    }

    pub fn resize(&mut self, size: Point) {
        let point = if size.x > size.y {
            Point::new(2. * size.x / size.y, 1.5)
        } else {
            Point::new(2., 1.5 * size.y / size.x)
        };
        self.min = point * Point::new(-1., -1.);
        self.max = point;
        self.size = size;
    }

    pub fn iter(&self) -> DomainIterator {
        DomainIterator::new(&self)
    }

    fn project(&self, point: Point) -> Point {
        self.min + point * (self.max - self.min) / self.size
    }

    pub fn change(&mut self, min: Point, max: Point) {
        self.min = min;
        self.max = max;
    }

    pub fn shift(&mut self, point: Point) {
        let projected_shift = self.project(point) - self.min;
        self.min -= projected_shift;
        self.max -= projected_shift;
    }

    pub fn scale(&mut self, factor: Point, center: Point) {
        let one = Point::new(1., 1.);
        let current_range = self.max - self.min;
        let shift = (self.project(center) - self.min) / current_range;

        self.min -= (current_range * (factor - one)) * shift;
        self.max += (current_range * (factor - one)) * (one - shift);
    }
}
