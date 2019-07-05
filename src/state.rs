use std::collections::HashMap;

use crate::core;
use crate::geometry::Point2D;
use crate::grid::{Grid, GridCell};
use crate::robot::{Robot, RotationDirection};

static FUEL_INITIAL_VALUE: u16 = 50;

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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
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

impl Action {
    pub fn move_point(&self) -> Option<Point2D> {
        match self {
            Action::MoveLeft => Some(Point2D::new(-1, 0)),
            Action::MoveUp => Some(Point2D::new(0, 1)),
            Action::MoveRight => Some(Point2D::new(1, 0)),
            Action::MoveDown => Some(Point2D::new(0, -1)),
            _ => None,
        }
    }

    pub fn rotation_direction(&self) -> Option<RotationDirection> {
        match self {
            Action::TurnCW => Some(RotationDirection::CW),
            Action::TurnCCW => Some(RotationDirection::CCW),
            _ => None,
        }
    }
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

    pub fn robot(&self, id: u32) -> &Robot {
        &self.robots[id as usize]
    }

    fn robot_mut(&mut self, id: u32) -> &mut Robot {
        &mut self.robots[id as usize]
    }

    pub fn max_points(&self) -> u32 {
        (1000.0 * ((self.grid.width * self.grid.height) as f64).log2()).ceil() as u32
    }

    pub fn has_wrappable_cells(&self) -> bool {
        self.grid.num_free > 0
    }

    fn maybe_collect_booster(&mut self, position: Point2D, reverse_action: &mut ReverseAction) {
        match self.boosters.remove(&position) {
            Some(type_) => {
                if type_ == BoosterType::X {
                    // put spawning point back
                    self.boosters.insert(position, type_);
                } else {
                    reverse_action.picked_up_booster = Some(type_);
                    reverse_action.picked_up_booster_position = position;
                    self.collected_boosters.insert(type_, self.collected_boosters[&type_] + 1);
                }
            }
            None => {}
        }
    }

    fn wrap(&mut self, reverse_action: &mut ReverseAction) {
        for robot in self.robots.iter() {
            assert!(!self.grid[robot.position].is_obstacle());
            for part in robot.get_visible_parts(&self.grid) {
                if self.grid[part] == GridCell::Free {
                    reverse_action.wrapped_points.insert(part, self.grid[part]);
                    self.grid.set(part, GridCell::Wrapped);
                }
            }
        }
    }

    fn unwrap(&mut self, reverse_action: &ReverseAction) {
        for (p, cell) in reverse_action.wrapped_points.iter() {
            self.grid.set(*p, *cell);
        }
    }
}

impl core::State for State {
    type Action = Action;
    type ReverseAction = ReverseAction;

    fn complete(&self) -> bool {
        !self.has_wrappable_cells()
    }

    fn agents_count(&self) -> u32 {
        self.robots.len() as u32
    }

    fn can_apply(&self, id: u32, action: Action) -> bool {
        use crate::state::Action::*;
        let robot = &self.robots[id as usize];
        match action {
            MoveLeft | MoveUp | MoveRight | MoveDown => {
                let new_position = robot.position + action.move_point().unwrap();
                self.grid.contains(new_position) && !self.grid[new_position].is_obstacle()
            }
            TurnCW | TurnCCW => true,
            Attach(location) => {
                self.collected_boosters[&BoosterType::B] > 0
                        && robot.tentacles.iter()
                            .map(|p| robot.orientation.apply_to(*p))
                            .map(|p| p.manhattan_dist(location))
                            .min() == Some(1)
            }
            Accelerate => self.collected_boosters[&BoosterType::F] > 0,
            Clone => {
                self.collected_boosters[&BoosterType::C] > 0
                        && self.boosters.get(&robot.position) == Some(&BoosterType::X)
            }
            NoOp => true,
            _ => unimplemented!("Not supported yet")
        }
    }

    fn apply(&mut self, id: u32, action: Action) -> ReverseAction {
        let mut reverse_action = ReverseAction {
            action,
            picked_up_booster: None,
            picked_up_booster_position: Point2D::new(0, 0),
            wrapped_points: HashMap::new(),
            robot_fuel_left: 0,
            made_two_moves: false,
            teleported_from: Point2D::new(0, 0),
        };
        self.maybe_collect_booster(self.robot(id).position, &mut reverse_action);
        reverse_action.robot_fuel_left = self.robot(id).fuel_left;

        use crate::state::Action::*;
        match action {
            MoveLeft | MoveUp | MoveRight | MoveDown => {
                self.robot_mut(id).position = self.robot(id).position + action.move_point().unwrap();
                assert!(!self.grid[self.robot(id).position].is_obstacle());
                self.wrap(&mut reverse_action);
                if self.robot(id).fuel_left > 0 {
                    self.maybe_collect_booster(self.robot(id).position, &mut reverse_action);
                    let new_position = self.robot(id).position + action.move_point().unwrap();
                    if self.grid.contains(new_position) && !self.grid[new_position].is_obstacle() {
                        self.robot_mut(id).position = new_position;
                        self.wrap(&mut reverse_action);
                        reverse_action.made_two_moves = true;
                    }
                }
            }
            TurnCW | TurnCCW => {
                self.robot_mut(id).rotate(action.rotation_direction().unwrap());
                self.wrap(&mut reverse_action);
            }
            Attach(location) => {
                let n = self.collected_boosters[&BoosterType::B];
                assert!(n > 0);
                self.collected_boosters.insert(BoosterType::B, n - 1);
                assert!(
                    self.robot(id).tentacles.iter()
                        .map(|p| self.robot(id).orientation.apply_to(*p))
                        .map(|p| p.manhattan_dist(location))
                        .min() == Some(1)
                );
                self.robot_mut(id).attach_tentacle(location);
                self.wrap(&mut reverse_action);
            }
            Accelerate => {
                let n = self.collected_boosters[&BoosterType::F];
                assert!(n > 0);
                self.collected_boosters.insert(BoosterType::F, n - 1);
                self.robot_mut(id).fuel_left += FUEL_INITIAL_VALUE;
                if self.robot(id).fuel_left == FUEL_INITIAL_VALUE {
                    self.robot_mut(id).fuel_left += 1 // accounting for decrement below
                }
            }
            Clone => {
                assert!(self.boosters[&self.robot(id).position] == BoosterType::X);
                let n = self.collected_boosters[&BoosterType::C];
                assert!(n > 0);
                self.collected_boosters.insert(BoosterType::C, n - 1);
                self.robots.push(Robot::new(self.robots.len() as u8, self.robot(id).position));
                self.wrap(&mut reverse_action);
            }
            InstallBeacon => {
                assert!(self.boosters[&self.robot(id).position] != BoosterType::X);
                let n = self.collected_boosters[&BoosterType::R];
                assert!(n > 0);
                self.collected_boosters.insert(BoosterType::R, n - 1);
                self.beacons.push(self.robot(id).position);
            }
            Teleport(location) => {
                assert!(self.beacons.contains(&location));
                reverse_action.teleported_from = self.robot(id).position;
                self.robot_mut(id).position = location;
                self.wrap(&mut reverse_action);
            }
            NoOp => {}
            _ => unimplemented!("Not supported yet"),
        }

        self.robot_mut(id).fuel_left = std::cmp::max(0, self.robot(id).fuel_left - 1);

        reverse_action
    }

    fn unapply(&mut self, id: u32, reverse_action: ReverseAction) -> Action {
        match reverse_action.picked_up_booster {
            Some(type_) => {
                self.boosters.insert(reverse_action.picked_up_booster_position, type_);
                self.collected_boosters.insert(type_, self.collected_boosters[&type_] - 1);
            }
            None => {}
        }

        use crate::state::Action::*;
        match reverse_action.action {
            MoveLeft | MoveUp | MoveRight | MoveDown => {
                self.robot_mut(id).position = self.robot(id).position - reverse_action.action.move_point().unwrap();
                if reverse_action.made_two_moves {
                    self.robot_mut(id).position = self.robot(id).position - reverse_action.action.move_point().unwrap();
                }
                self.unwrap(&reverse_action);
            }
            TurnCW | TurnCCW => {
                self.robot_mut(id).rotate(reverse_action.action.rotation_direction().unwrap());
                self.unwrap(&reverse_action);
            }
            Attach(_) => {
                self.collected_boosters.insert(BoosterType::B, self.collected_boosters[&BoosterType::B] + 1);
                self.robot_mut(id).detach_last_tentacle();
                self.unwrap(&reverse_action);
            }
            Accelerate => {
                self.collected_boosters.insert(BoosterType::F, self.collected_boosters[&BoosterType::F] + 1);
            }
            Clone => {
                self.collected_boosters.insert(BoosterType::C, self.collected_boosters[&BoosterType::C] + 1);
                self.robots.remove(self.robots.len() - 1);
                self.unwrap(&reverse_action);
            }
            InstallBeacon => {
                self.collected_boosters.insert(BoosterType::R, self.collected_boosters[&BoosterType::R] + 1);
                let position = self.robot(id).position;
                self.beacons.remove_item(&position);
            }
            Teleport(_) => {
                self.robot_mut(id).position = reverse_action.teleported_from;
                self.unwrap(&reverse_action);
            }
            NoOp => {}
            _ => unimplemented!("Not supported yet"),
        }
        
        self.robot_mut(id).fuel_left = reverse_action.robot_fuel_left;
        
        reverse_action.action
    }
}
