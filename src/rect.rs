use crate::{Circle, Point2, Size2};

pub struct Rect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl Rect {
    pub const ZERO: Self = Self::splat(0.);

    pub const fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
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

    pub fn from_circle(circle: Circle) -> Self {
        let diameter = 2. * circle.radius;
        Self::from_center(circle.center, Size2::new(diameter, diameter))
    }

    pub const fn splat(v: f32) -> Self {
        Self::new(v, v, v, v)
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

    pub fn origin(&self) -> Point2 {
        Point2::new(self.left, self.top)
    }

    pub fn center(&self) -> Point2 {
        Point2::new(
            (self.left + self.right) * 0.5,
            (self.top + self.bottom) * 0.5,
        )
    }

    pub fn area(&self) -> f32 {
        self.size().area()
    }

    pub fn size(&self) -> Size2 {
        Size2::new(self.max_x() - self.min_x(), self.max_y() - self.min_y())
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.size().aspect_ratio()
    }

    pub fn is_empty(&self) -> bool {
        self.size().is_empty()
    }

    pub fn contains(&self, point: Point2) -> bool {
        let Point2 { x, y } = point;
        x >= self.min_x() && x < self.max_x() && y >= self.min_y() && y < self.max_y()
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
