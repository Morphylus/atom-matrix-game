use std::f32::consts::PI;

use crate::matrix::RGB8;

#[derive(PartialEq, Clone, Copy)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

enum Direction {
    Stay,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

pub struct MiniGame {
    pub curr_pos: Position,
    fractional_x: f32,
    fractional_y: f32,
    movement_threshold: f32,
    velocity_x: f32,
    velocity_y: f32,
}

impl MiniGame {
    pub fn new(movement_threshold: f32) -> Self {
        MiniGame {
            curr_pos: Position { x: 2, y: 2 },
            movement_threshold,
            velocity_x: 0.0,
            velocity_y: 0.0,
            fractional_x: 2.0,
            fractional_y: 2.0,
        }
    }

    pub fn update_position_with_delta(&mut self, pitch: f32, roll: f32, delta_time: f32) {
        let next_dir = decide_next_dir(pitch, roll, self.movement_threshold);

        let (target_vel_x, target_vel_y) = match next_dir {
            Direction::North => (0.0, 1.0),
            Direction::NorthEast => (0.707, 0.707),
            Direction::East => (1.0, 0.0),
            Direction::SouthEast => (0.707, -0.707),
            Direction::South => (0.0, -1.0),
            Direction::SouthWest => (-0.707, -0.707),
            Direction::West => (-1.0, 0.0),
            Direction::NorthWest => (-0.707, 0.707),
            Direction::Stay => (0.0, 0.0),
        };

        self.fractional_x = (self.fractional_x + target_vel_x * delta_time)
            .max(0.0)
            .min(4.0);
        self.fractional_y = (self.fractional_y + target_vel_y * delta_time)
            .max(0.0)
            .min(4.0);

        self.curr_pos = Position {
            x: self.fractional_x as u8,
            y: self.fractional_y as u8,
        };
    }

    pub fn update_position(&mut self, pitch: f32, roll: f32) {
        let next_dir = decide_next_dir(pitch, roll, self.movement_threshold);
        self.curr_pos = decide_next_pos(self.curr_pos, next_dir);
    }
}

fn decide_next_dir(pitch: f32, roll: f32, movement_threshold: f32) -> Direction {
    let magnitude = (pitch.powi(2) + roll.powi(2)).sqrt();
    let angle = pitch.atan2(roll) * 180.0 / PI;

    if magnitude < movement_threshold {
        return Direction::Stay;
    }

    if angle >= -22.5 && angle < 22.5 {
        return Direction::East;
    } else if angle >= 22.5 && angle < 67.5 {
        return Direction::NorthEast;
    } else if angle >= 67.5 && angle < 112.5 {
        return Direction::North;
    } else if angle >= 112.5 && angle < 157.5 {
        return Direction::NorthWest;
    } else if angle >= -67.5 && angle < -22.5 {
        return Direction::SouthEast;
    } else if angle >= -112.5 && angle < -67.5 {
        return Direction::South;
    } else if angle >= -157.5 && angle < -112.5 {
        return Direction::SouthWest;
    } else {
        return Direction::West;
    }
}

fn decide_next_pos(curr_pos: Position, direction: Direction) -> Position {
    let new_pos = match direction {
        Direction::North => Position {
            x: curr_pos.x,
            y: if curr_pos.y < 4 {
                curr_pos.y + 1
            } else {
                curr_pos.y
            },
        },
        Direction::NorthEast => Position {
            x: if curr_pos.x < 4 {
                curr_pos.x + 1
            } else {
                curr_pos.x
            },
            y: if curr_pos.y < 4 {
                curr_pos.y + 1
            } else {
                curr_pos.y
            },
        },
        Direction::East => Position {
            x: if curr_pos.x < 4 {
                curr_pos.x + 1
            } else {
                curr_pos.x
            },
            y: curr_pos.y,
        },
        Direction::SouthEast => Position {
            x: if curr_pos.x < 4 {
                curr_pos.x + 1
            } else {
                curr_pos.x
            },
            y: if curr_pos.y > 0 {
                curr_pos.y - 1
            } else {
                curr_pos.y
            },
        },
        Direction::South => Position {
            x: curr_pos.x,
            y: if curr_pos.y > 0 {
                curr_pos.y - 1
            } else {
                curr_pos.y
            },
        },
        Direction::SouthWest => Position {
            x: if curr_pos.x > 0 {
                curr_pos.x - 1
            } else {
                curr_pos.x
            },
            y: if curr_pos.y > 0 {
                curr_pos.y - 1
            } else {
                curr_pos.y
            },
        },
        Direction::West => Position {
            x: if curr_pos.x > 0 {
                curr_pos.x - 1
            } else {
                curr_pos.x
            },
            y: curr_pos.y,
        },
        Direction::NorthWest => Position {
            x: if curr_pos.x > 0 {
                curr_pos.x - 1
            } else {
                curr_pos.x
            },
            y: if curr_pos.y < 4 {
                curr_pos.y + 1
            } else {
                curr_pos.y
            },
        },
        Direction::Stay => Position {
            x: curr_pos.x,
            y: curr_pos.y,
        },
    };

    new_pos
}
