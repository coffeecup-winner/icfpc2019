use std::ops;

use crate::geometry::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum GridCell {
    Obstacle,
    Void,
    Wrapped,
    Free,
}

impl GridCell {
    pub fn is_obstacle(&self) -> bool {
        match self {
            GridCell::Obstacle | GridCell::Void => true,
            _ => false
        }
    }
}

pub struct Grid {
    pub width: u16,
    pub height: u16,
    grid: Vec<GridCell>,
    pub num_obstacles: u32,
    pub num_void: u32,
    pub num_wrapped: u32,
    pub num_free: u32,
}

impl Grid {
    pub fn new(width: u16, height: u16, initial_value: GridCell) -> Grid {
        let size = (width * height) as u32;
        Grid {
            width,
            height,
            grid: vec![initial_value; size as usize],
            num_obstacles: if initial_value == GridCell::Obstacle { size } else { 0 },
            num_void: if initial_value == GridCell::Void { size } else { 0 },
            num_wrapped: if initial_value == GridCell::Wrapped { size } else { 0 },
            num_free: if initial_value == GridCell::Free { size } else { 0 },
        }
    }

    pub fn set(&mut self, p: Point2D, value: GridCell) {
        match self[p] {
            GridCell::Obstacle => self.num_obstacles -= 1,
            GridCell::Void => self.num_void -= 1,
            GridCell::Wrapped => self.num_wrapped -= 1,
            GridCell::Free => self.num_free -= 1,
        }
        self.grid[(p.y * self.width as i32 + p.x) as usize] = value;
        match value {
            GridCell::Obstacle => self.num_obstacles += 1,
            GridCell::Void => self.num_void += 1,
            GridCell::Wrapped => self.num_wrapped += 1,
            GridCell::Free => self.num_free += 1,
        }
    }

    pub fn contains(&self, p: Point2D) -> bool {
        p.x >= 0 && p.x < self.width as i32 && p.y >= 0 && p.y < self.height as i32
    }
}

impl ops::Index<Point2D> for Grid {
    type Output = GridCell;

    fn index(&self, p: Point2D) -> &Self::Output {
        &self.grid[(p.y * self.width as i32 + p.x) as usize]
    }
}

#[test]
fn test_grid() {
    let mut grid = Grid::new(2, 2, GridCell::Obstacle);
    assert_eq!(grid.width, 2);
    assert_eq!(grid.height, 2);
    assert_eq!(grid.num_obstacles, 4);
    assert_eq!(grid.num_void, 0);
    assert_eq!(grid.num_wrapped, 0);
    assert_eq!(grid.num_free, 0);
    assert_eq!(grid.contains(Point2D::new(0, 0)), true);
    assert_eq!(grid.contains(Point2D::new(-1, 0)), false);
    assert_eq!(grid.contains(Point2D::new(0, -1)), false);
    assert_eq!(grid.contains(Point2D::new(1, 1)), true);
    assert_eq!(grid.contains(Point2D::new(2, 1)), false);
    assert_eq!(grid.contains(Point2D::new(1, 2)), false);
    grid.set(Point2D::new(0, 1), GridCell::Void);
    grid.set(Point2D::new(1, 0), GridCell::Wrapped);
    grid.set(Point2D::new(1, 1), GridCell::Free);
    assert_eq!(grid[Point2D::new(0, 0)], GridCell::Obstacle);
    assert_eq!(grid[Point2D::new(0, 1)], GridCell::Void);
    assert_eq!(grid[Point2D::new(1, 0)], GridCell::Wrapped);
    assert_eq!(grid[Point2D::new(1, 1)], GridCell::Free);
    assert_eq!(grid.num_obstacles, 1);
    assert_eq!(grid.num_void, 1);
    assert_eq!(grid.num_wrapped, 1);
    assert_eq!(grid.num_free, 1);
}
