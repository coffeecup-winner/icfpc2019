use std::cmp;
use std::collections::HashMap;

use regex::Regex;

use crate::geometry::Point2D;
use crate::grid::{Grid, GridCell};
use crate::state::{Booster, BoosterType, State};

pub struct Problem;

impl Problem {
    pub fn parse(mut s: String) -> State {
        let raw_parts = s.split('#').collect::<Vec<&str>>();
        assert_eq!(raw_parts.len(), 4);
        let raw_map = raw_parts[0];
        let raw_initial = raw_parts[1];
        let raw_obstacles = raw_parts[2];
        let raw_boosters = raw_parts[3];
        
        let obstacles = raw_obstacles.split(';')
            .collect::<Vec<&str>>()
            .into_iter()
            .filter(|s| s.len() > 0)
            .map(|s| Poly::new(Self::parse_points(s)))
            .collect::<Vec<_>>();
        let boosters = raw_boosters.split(';')
            .collect::<Vec<&str>>()
            .into_iter()
            .filter(|s| s.len() > 0)
            .map(|s| Self::parse_booster(s))
            .collect::<Vec<_>>();
        let map = Poly::new(Self::parse_points(raw_map));
        let initial_position = Self::parse_point(raw_initial);

        let (bottom_left, top_right) = map.bbox();
        assert_eq!(bottom_left.x, 0);
        assert_eq!(bottom_left.y, 0);

        let width = top_right.y;
        let height = top_right.x;

        let mut grid = Grid::new(width as u16, height as u16, GridCell::Void);
        map.project(&mut grid, GridCell::Free);
        obstacles.iter().for_each(|o| {
            o.project(&mut grid, GridCell::Obstacle);
        });

        State::new(grid, boosters, initial_position)
    }

    fn parse_booster(s: &str) -> Booster {
        let type_ = match s.chars().nth(0).unwrap() {
            'B' => BoosterType::B,
            'C' => BoosterType::C,
            'F' => BoosterType::F,
            'L' => BoosterType::L,
            'R' => BoosterType::R,
            'X' => BoosterType::X,
            _ => panic!("Invalid booster type"),
        };
        Booster {
            type_,
            position: Self::parse_point(&s[1..]),
        }
    }

    fn parse_points(s: &str) -> Vec<Point2D> {
        lazy_static! {
            static ref POINTS_REGEX: Regex = Regex::new(r"\((\d+),(\d+)\)(?:,|$)").unwrap();
        }
        POINTS_REGEX.captures_iter(s).map(|c|
            Point2D::new(
                c[1].parse::<i32>().unwrap(),
                c[2].parse::<i32>().unwrap()
            )
        ).collect()
    }

    fn parse_point(s: &str) -> Point2D {
        lazy_static! {
            static ref POINT_REGEX: Regex = Regex::new(r"^\((\d+),(\d+)\)$").unwrap();
        }
        let captures = POINT_REGEX.captures(s).unwrap();
        Point2D::new(
            captures[1].parse::<i32>().unwrap(),
            captures[2].parse::<i32>().unwrap()
        )
    }
}

#[test]
fn test_parsing() {
    assert_eq!(Problem::parse_point("(0,1)"), Point2D::new(0, 1));
    assert_eq!(Problem::parse_points("(0,1),(1,0)"), vec!(Point2D::new(0, 1), Point2D::new(1, 0)));
    assert_eq!(Problem::parse_booster("B(1,0)"), Booster { type_: BoosterType::B, position: Point2D::new(1, 0) });
}

struct Poly {
    contour: Vec<Point2D>
}

impl Poly {
    pub fn new(contour: Vec<Point2D>) -> Poly {
        Poly {
            contour
        }
    }

    pub fn bbox(&self) -> (Point2D, Point2D) {
        let mut min_x = i32::max_value();
        let mut max_x = 0;
        let mut min_y = i32::max_value();
        let mut max_y = 0;
        for p in self.contour.iter() {
            min_x = cmp::min(min_x, p.x);
            max_x = cmp::max(max_x, p.x);
            min_y = cmp::min(min_y, p.y);
            max_y = cmp::max(max_y, p.y);
        }
        (Point2D::new(min_x, min_y), Point2D::new(max_x, max_y))
    }

    pub fn contains(&self, p: Point2D) -> bool {
        let mut count = 0;
        for i in 0..self.contour.len() {
            let a = self.contour[i];
            let b = self.contour[(i + 1) % self.contour.len()];
            if a.x == b.x {
                if p.x < a.x && (a.y..b.y).contains(&p.y) {
                    count += 1;
                }
            } else {
                assert_eq!(a.y, b.y);
            }
        }
        count % 2 == 0
    }

    pub fn project(&self, grid: &mut Grid, cell: GridCell) {
        let mut verticals: HashMap<i32, Vec<Vertical>> = HashMap::new();
        for i in 0..self.contour.len() {
            let a = self.contour[i];
            let b = self.contour[(i + 1) % self.contour.len()];
            if a.x == b.x {
                if !verticals.contains_key(&a.x) {
                    verticals.insert(a.x, vec![]);
                }
                verticals.get_mut(&a.x).unwrap().push(Vertical { x: a.x, min_y: cmp::min(a.y, b.y), max_y: cmp::max(a.y, b.y) })
            }
        }

        for y in 0..grid.width as i32 {
            let mut count = 0;
            for x in 0..grid.height as i32 {
                match verticals.get(&x) {
                    Some(vs) => {
                        for v in vs.iter() {
                            if (2 * v.min_y..2 * v.max_y).contains(&(2 * y + 1)) {
                                count += 1;
                            }
                        }
                    }
                    None => {}
                }

                if count % 2 > 0 {
                    grid.set(Point2D::new(x, y), cell);
                }
            }
        }
    }
}

struct Vertical {
    x: i32,
    min_y: i32,
    max_y: i32,
}
