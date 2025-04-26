#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        ThrowObject {
            init_position: init_position.clone(),
            init_velocity: init_velocity.clone(),
            actual_position: init_position,
            actual_velocity: init_velocity,
            time: 0.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;

    fn next(&mut self) -> Option<Self::Item> {
        if self.actual_position.y <= 0.0 {
            return None;
        }

        self.time += 1.0;

        let new_x = self.init_position.x + self.init_velocity.x * self.time;
        let new_y = (self.init_position.y + self.init_velocity.y * self.time - 0.5 * 9.8 * self.time.powi(2)).round_to(1);

        let new_vx = self.init_velocity.x;
        let new_vy = (self.init_velocity.y - 9.8 * self.time).round_to(1);

        self.actual_position = Object { x: new_x, y: new_y.max(0.0) };
        self.actual_velocity = Object { x: new_vx, y: new_vy };

        if self.actual_position.y <= 0.0 {
            None
        } else {
            Some(self.clone())
        }
    }
}

trait RoundTo {
    fn round_to(&self, decimals: u32) -> f32;
}

impl RoundTo for f32 {
    fn round_to(&self, decimals: u32) -> f32 {
        let factor = 10.0f32.powi(decimals as i32);
        (self * factor).round() / factor
    }
}