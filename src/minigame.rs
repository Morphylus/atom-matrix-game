#[derive(PartialEq, Clone, Copy)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

pub struct MiniGame {
    pub curr_pos: Position,
    fractional_x: f32,
    fractional_y: f32,
    movement_threshold: f32,
}

impl MiniGame {
    pub fn new(movement_threshold: f32) -> Self {
        MiniGame {
            curr_pos: Position { x: 2, y: 2 },
            movement_threshold,
            fractional_x: 2.0,
            fractional_y: 2.0,
        }
    }

    pub fn update_position_with_delta(&mut self, pitch: f32, roll: f32, delta_time: f32) {
        let magnitude = (pitch.powi(2) + roll.powi(2)).sqrt();

        if magnitude < self.movement_threshold {
            return;
        }

        let normalized_pitch = pitch / magnitude;
        let normalized_roll = roll / magnitude;

        self.fractional_x = (self.fractional_x + normalized_roll * delta_time)
            .max(0.0)
            .min(4.0);
        self.fractional_y = (self.fractional_y + normalized_pitch * delta_time)
            .max(0.0)
            .min(4.0);

        self.curr_pos = Position {
            x: self.fractional_x as u8,
            y: self.fractional_y as u8,
        };
    }
}
