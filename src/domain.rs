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
    pub origin: Point,
    pub scale: f64,
    pub size: Point, // contains canvas width and height
}

impl Domain {
    pub fn new(size: Point) -> Domain {
        let origin = Point::new(0., 0.);
        let scale = 1.;
        Domain {
            size,
            origin,
            scale,
        }
    }

    pub fn resize(&mut self, size: Point) {
        self.size = size;
    }

    pub fn iter(&self) -> DomainIterator {
        DomainIterator::new(&self)
    }

    fn scale_point(&self) -> Point {
        if self.size.x > self.size.y {
            Point::new(self.scale * (self.size.x / self.size.y), self.scale)
        } else {
            Point::new(self.scale, self.scale * (self.size.y / self.size.x))
        }
    }

    fn projected_origin(&self) -> Point {
        self.origin - self.scale_point()
    }

    fn project(&self, point: Point) -> Point {
        let two = Point::new(2., 2.);
        self.projected_origin() + point * two * self.scale_point() / self.size
    }

    pub fn change(&mut self, origin: Point, scale: f64) {
        self.origin = origin;
        self.scale = scale;
    }

    pub fn shift(&mut self, point: Point) {
        self.origin += self.project(point) - self.projected_origin()
    }

    pub fn zoom(&mut self, factor: f64, center: Point) {
        let delta = self.project(Point::new(factor, factor)) - self.projected_origin();

        self.origin -=
            delta * (self.project(center) - self.origin) / Point::new(self.scale, self.scale);

        self.scale += delta.x.min(delta.y);
    }
}
