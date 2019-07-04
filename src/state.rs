use std::collections::HashMap;

use crate::geometry::Point2D;
use crate::grid::{Grid, GridCell};
use crate::robot::Robot;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum BoosterType {
    B,
    F,
    L,
    X,
    R,
    C,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Booster {
    pub type_: BoosterType,
    pub position: Point2D,
}

pub enum Action {
    MoveLeft,
    MoveUp,
    MoveRight,
    MoveDown,
    TurnCW,
    TurnCCW,
    Attach(Point2D),
    Clone,
    Accelerate,
    Drill,
    NoOp,
    InstallBeacon,
    Teleport(Point2D),
}

pub struct ReverseAction {
    pub action: Action,
    pub picked_up_booster: Option<BoosterType>,
    pub picked_up_booster_position: Point2D, // TODO: merge with above
    pub wrapped_points: HashMap<Point2D, GridCell>,
    pub robot_fuel_left: u16,
    pub made_two_moves: bool,
    pub teleported_from: Point2D,
}

pub struct State {
    grid: Grid,
    boosters: HashMap<Point2D, BoosterType>,
    robots: Vec<Robot>,
    beacons: Vec<Point2D>,
    collected_boosters: HashMap<BoosterType, u8>,
}

impl State {
    pub fn new(grid: Grid, boosters: Vec<Booster>, initial_position: Point2D) -> State {
        State {
            grid,
            boosters: boosters.into_iter().map(|b| (b.position, b.type_)).collect(),
            robots: vec![Robot::new(0, initial_position)],
            beacons: vec![],
            collected_boosters: HashMap::new(),
        }
    }

    pub fn max_points(&self) -> u32 {
        (1000.0 * ((self.grid.width * self.grid.height) as f64).log2()).ceil() as u32
    }

    pub fn has_wrappable_cells(&self) -> bool {
        self.grid.num_free > 0
    }
}
