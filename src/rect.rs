use crate::{Circle, Point, Size};

pub struct Rect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Rect {
    pub const ZERO: Self = Self::new(0., 0., 0., 0.);

    pub const fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn from_center(center: Point, size: Size) -> Self {
        let Point { x, y } = center;
        let Size { width, height } = size.scale(0.5);
        Self::new(x - width, y - height, x + width, y + height)
    }

    pub fn from_origin(origin: Point, size: Size) -> Self {
        let Point { x, y } = origin;
        let Size { width, height } = size;
        Self::new(x, y, x + width, y + height)
    }

    pub fn from_points(p0: Point, p1: Point) -> Self {
        Self::new(
            p0.x.min(p1.x),
            p0.y.min(p1.y),
            p0.x.max(p1.x),
            p0.y.max(p1.y),
        )
    }

    pub fn from_circle(circle: Circle) -> Self {
        let diameter = circle.radius * 2.0;
        Self::from_center(circle.center, Size::new(diameter, diameter))
    }

    pub fn min_x(&self) -> f32 {
        self.left.min(self.right)
    }

    pub fn max_x(&self) -> f32 {
        self.left.max(self.right)
    }

    pub fn min_y(&self) -> f32 {
        self.top.min(self.bottom)
    }

    pub fn max_y(&self) -> f32 {
        self.top.max(self.bottom)
    }

    pub fn origin(&self) -> Point {
        Point::new(self.left, self.top)
    }

    pub fn center(&self) -> Point {
        Point::new(
            (self.left + self.right) * 0.5,
            (self.top + self.bottom) * 0.5,
        )
    }

    pub fn area(&self) -> f32 {
        self.size().area()
    }

    pub fn size(&self) -> Size {
        Size::new(self.max_x() - self.min_x(), self.max_y() - self.min_y())
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.size().aspect_ratio()
    }

    pub fn is_empty(&self) -> bool {
        self.size().is_empty()
    }

    pub fn contains(&self, point: Point) -> bool {
        let Point { x, y } = point;
        x >= self.left && x < self.right && y >= self.top && y < self.bottom
    }

    pub fn abs(&self) -> Self {
        Self::new(self.min_x(), self.min_y(), self.max_x(), self.max_y())
    }

    pub fn scale(&self, factor: f32) -> Self {
        Self::from_origin(self.origin(), self.size().scale(factor))
    }

    pub fn intersect(&self, other: Self) -> Option<Self> {
        if self.max_x() <= other.min_x() || other.max_x() <= self.min_x() {
            return None;
        }
        if self.max_y() <= other.min_y() || other.max_y() <= self.min_y() {
            return None;
        }
        None
    }

    pub fn inflate(&self, width: f32, height: f32) -> Self {
        Self::new(
            self.left - width,
            self.top - height,
            self.right + width,
            self.bottom + height,
        )
    }

    pub fn deflate(&self, width: f32, height: f32) -> Self {
        self.inflate(-width, -height)
    }

    // pub fn
}
