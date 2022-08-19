use crate::{Circle, Numeric, Point2, Size};

pub struct Rect<N: Numeric> {
    pub left: N,
    pub top: N,
    pub right: N,
    pub bottom: N,
}

impl<N: Numeric> Rect<N> {
    pub const ZERO: Self = Self::splat(N::ZERO);

    pub const fn new(left: N, top: N, right: N, bottom: N) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn from_center(center: Point2<N>, size: Size<N>) -> Self {
        let Point2 { x, y } = center;
        let Size { width, height } = size.scale(0.5);
        Self::new(x - width, y - height, x + width, y + height)
    }

    pub fn from_origin(origin: Point2<N>, size: Size<N>) -> Self {
        let Point2 { x, y } = origin;
        let Size { width, height } = size;
        Self::new(x, y, x + width, y + height)
    }

    pub fn from_point2s(p0: Point2<N>, p1: Point2<N>) -> Self {
        Self::new(
            p0.x.min(p1.x),
            p0.y.min(p1.y),
            p0.x.max(p1.x),
            p0.y.max(p1.y),
        )
    }

    pub fn from_circle(circle: Circle<N>) -> Self {
        let diameter = circle.radius * 2.0.into();
        Self::from_center(circle.center, Size::new(diameter, diameter))
    }

    pub const fn splat(v: N) -> Self {
        Self::new(v, v, v, v)
    }

    pub fn min_x(&self) -> N {
        self.left.min(self.right)
    }

    pub fn max_x(&self) -> N {
        self.left.max(self.right)
    }

    pub fn min_y(&self) -> N {
        self.top.min(self.bottom)
    }

    pub fn max_y(&self) -> N {
        self.top.max(self.bottom)
    }

    pub fn origin(&self) -> Point2<N> {
        Point2::new(self.left, self.top)
    }

    pub fn center(&self) -> Point2<N> {
        Point2::new(
            (self.left + self.right) * 0.5.into(),
            (self.top + self.bottom) * 0.5.into(),
        )
    }

    pub fn area(&self) -> N {
        self.size().area()
    }

    pub fn size(&self) -> Size<N> {
        Size::new(self.max_x() - self.min_x(), self.max_y() - self.min_y())
    }

    pub fn aspect_ratio(&self) -> N {
        self.size().aspect_ratio()
    }

    pub fn is_empty(&self) -> bool {
        self.size().is_empty()
    }

    pub fn contains(&self, point2: Point2<N>) -> bool {
        let Point2 { x, y } = point2;
        x >= self.left && x < self.right && y >= self.top && y < self.bottom
    }

    pub fn abs(&self) -> Self {
        Self::new(self.min_x(), self.min_y(), self.max_x(), self.max_y())
    }

    pub fn scale(&self, factor: N) -> Self {
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

    pub fn inflate(&self, width: N, height: N) -> Self {
        Self::new(
            self.left - width,
            self.top - height,
            self.right + width,
            self.bottom + height,
        )
    }

    pub fn deflate(&self, width: N, height: N) -> Self {
        self.inflate(-width, -height)
    }

    // pub fn
}
