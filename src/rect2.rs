use crate::{Circle2, Point2, Size2};

pub struct Rect2 {
    pub min: Point2,
    pub max: Point2,
}

impl Rect2 {
    pub const ZERO: Self = Self::splat(0.);

    pub const fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        Self {
            min: Point2::new(min_x, min_y),
            max: Point2::new(max_x, max_y),
        }
    }

    pub fn from_center(center: Point2, size: Size2) -> Self {
        let Point2 { x, y } = center;
        let Size2 { width, height } = size.scale(0.5);
        Self::new(x - width, y - height, x + width, y + height)
    }

    pub fn from_origin(origin: Point2, size: Size2) -> Self {
        let Point2 { x, y } = origin;
        let Size2 { width, height } = size;
        Self::new(x, y, x + width, y + height)
    }

    pub fn from_points(p0: Point2, p1: Point2) -> Self {
        Self::new(
            p0.x.min(p1.x),
            p0.y.min(p1.y),
            p0.x.max(p1.x),
            p0.y.max(p1.y),
        )
    }

    pub fn from_circle(circle: Circle2) -> Self {
        let diameter = 2. * circle.radius;
        Self::from_center(circle.center, Size2::new(diameter, diameter))
    }

    pub const fn splat(v: f32) -> Self {
        Self::new(v, v, v, v)
    }

    pub fn origin(&self) -> Point2 {
        self.min.clone()
    }

    pub fn center(&self) -> Point2 {
        Point2::new(
            self.min.x + ((self.max.x - self.min.x) * 0.5),
            self.min.y + ((self.max.y + self.min.y) * 0.5),
        )
    }

    pub fn area(&self) -> f32 {
        self.size().area()
    }

    pub fn size(&self) -> Size2 {
        Size2::new(self.max.x - self.min.x, self.max.y - self.min.y)
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.size().aspect_ratio()
    }

    pub fn is_empty(&self) -> bool {
        self.size().is_empty()
    }

    pub fn contains_point(&self, point: Point2) -> bool {
        let Point2 { x, y } = point;
        x >= self.min.x && x < self.max.x && y >= self.min.y && y < self.max.y
    }

    pub fn abs(&self) -> Self {
        Self::from_points(self.min.abs(), self.max.abs())
    }

    pub fn scale(&self, factor: f32) -> Self {
        Self::from_origin(self.origin(), self.size().scale(factor))
    }

    pub fn intersection(&self, other: Self) -> Option<Self> {
        if self.max.x <= other.min.x || other.max.x <= self.min.x {
            return None;
        }
        if self.max.y <= other.min.y || other.max.y <= self.min.y {
            return None;
        }

        //TODO: return valid intersection
        None
    }

    pub fn inflate(&self, delta_x: f32, delta_y: f32) -> Self {
        Self::new(
            self.min.x - delta_x,
            self.min.y - delta_y,
            self.max.x + delta_x,
            self.max.y + delta_y,
        )
    }

    pub fn deflate(&self, width: f32, height: f32) -> Self {
        self.inflate(-width, -height)
    }

    // pub fn
}
