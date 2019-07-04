use crate::geometry::{Orientation, Point2D};

pub struct Robot {
    pub id: u8,
    pub position: Point2D,
    pub tentacles: Vec<Point2D>,
    pub orientation: Orientation,
    pub fuel_left: u16,
}

impl Robot {
    pub fn new(id: u8, position: Point2D) -> Robot {
        Robot {
            id,
            position,
            tentacles: vec![],
            orientation: Orientation::Right,
            fuel_left: 0,
        }
    }
}
