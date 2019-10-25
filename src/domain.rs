use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub struct DomainIterator<'a> {
    min: &'a Point,
    max: &'a Point,
    width: usize,
    height: usize,
    x: usize,
    y: usize,
}

impl<'a> Iterator for DomainIterator<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        if self.x >= self.width {
            self.x = 0;
            self.y -= 1;
            if self.y == 0 {
                return None;
            }
        }
        self.x += 1;

        Some(Point {
            x: self.min.x + (self.x as f64) * (self.max.x - self.min.x) / (self.width as f64),
            y: self.min.y + (self.y as f64) * (self.max.y - self.min.y) / (self.height as f64),
        })
    }
}

pub struct Domain {
    pub min: Point,
    pub max: Point,
    pub width: usize,
    pub height: usize,
}

impl Domain {
    pub fn iter(&self) -> DomainIterator {
        DomainIterator {
            x: 0,
            y: self.height,
            width: self.width,
            height: self.height,
            min: &self.min,
            max: &self.max,
        }
    }
}
