use std::ops;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
}

impl Point2D {
    pub fn new(x: i32, y: i32) -> Point2D {
        return Point2D {
            x,
            y,
        }
    }

    pub fn manhattan_dist(&self, p: Point2D) -> i32 {
        (self.x - p.x).abs() + (self.y - p.y).abs()
    }
}

impl ops::Neg for Point2D {
    type Output = Point2D;

    fn neg(self) -> Point2D {
        Point2D::new(-self.x, -self.y)
    }
}

impl ops::Add for Point2D {
    type Output = Point2D;

    fn add(self, rhs: Point2D) -> Point2D {
        Point2D::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Sub for Point2D {
    type Output = Point2D;

    fn sub(self, rhs: Point2D) -> Point2D {
        Point2D::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl ops::Mul for Point2D {
    type Output = i32;

    fn mul(self, rhs: Point2D) -> i32 {
        self.x * rhs.x + self.y * rhs.y
    }
}

#[test]
fn test_point2d() {
    assert_eq!(-Point2D::new(-1, 1), Point2D::new(1, -1));
    assert_eq!(Point2D::new(-1, 1) + Point2D::new(1, 1), Point2D::new(0, 2));
    assert_eq!(Point2D::new(1, 1) - Point2D::new(1, -1), Point2D::new(0, 2));
    assert_eq!(Point2D::new(2, 3) * Point2D::new(-2, 4), 8);
}
