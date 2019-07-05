use crate::geometry::Point2D;
use crate::grid::Grid;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum RotationDirection {
    CW,
    CCW,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Orientation {
    Left,
    Up,
    Right,
    Down,
}

impl Orientation {
    pub fn rotate(&self, direction: RotationDirection) -> Orientation {
        match direction {
            RotationDirection::CW => match self {
                Orientation::Left => Orientation::Up,
                Orientation::Up => Orientation::Right,
                Orientation::Right => Orientation::Down,
                Orientation::Down => Orientation::Left,
            },
            RotationDirection::CCW => match self {
                Orientation::Left => Orientation::Down,
                Orientation::Up => Orientation::Left,
                Orientation::Right => Orientation::Up,
                Orientation::Down => Orientation::Right,
            },
        }
    }

    pub fn apply_to(&self, p: Point2D) -> Point2D {
        let (ax, ay) = self.multiplier();
        Point2D::new(p * ax, p * ay)
    }

    pub fn unapply_to(&self, p: Point2D) -> Point2D {
        let (ax, ay) = self.multiplier();
        Point2D::new(p * -ax, p * -ay)
    }

    fn multiplier(&self) -> (Point2D, Point2D) {
        match self {
            Orientation::Left => (
                Point2D::new(-1, 0),
                Point2D::new(0, -1)
            ),
            Orientation::Up => (
                Point2D::new(0, -1),
                Point2D::new(1, 0)
            ),
            Orientation::Right => (
                Point2D::new(1, 0),
                Point2D::new(0, 1)
            ),
            Orientation::Down => (
                Point2D::new(0, 1),
                Point2D::new(-1, 0)
            )
        }
    }
}

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
            tentacles: vec![
                Point2D::new(1, 1),
                Point2D::new(1, -1),
                Point2D::new(1, 0),
            ],
            orientation: Orientation::Right,
            fuel_left: 0,
        }
    }

    pub fn rotate(&mut self, direction: RotationDirection) {
        self.orientation = self.orientation.rotate(direction);
    }

    pub fn get_visible_parts(&self, grid: &Grid) -> Vec<Point2D> {
        // TODO: this is a quick and dirty visibility check
        //   4
        //   3
        //  021 <- tentacle indices, the algorithm depends on this order
        //   R

        let mut result = Vec::new();
        result.push(self.position);

        for t in self.tentacles.iter().take(2) {
            let p = self.orientation.apply_to(*t) + self.position;
            if grid.contains(p) && !grid[p].is_obstacle() {
                result.push(p);
            }
        }

        for t in self.tentacles.iter().skip(2) {
            let p = self.orientation.apply_to(*t) + self.position;
            if grid.contains(p) && !grid[p].is_obstacle() {
                result.push(p);
            } else {
                break
            }
        }
        result
    }

    pub fn attach_tentacle(&mut self, p: Point2D) {
        self.tentacles.push(self.orientation.unapply_to(p));
    }

    pub fn detach_last_tentacle(&mut self) {
        self.tentacles.remove(self.tentacles.len() - 1);
    }

    pub fn next_attachment_point(&self) -> Point2D {
        let x = self.tentacles.iter().map(|t| t.x).max().unwrap();
        self.orientation.apply_to(Point2D::new(x + 1, 0))
    }
}
