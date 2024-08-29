#[derive(Debug, Clone, Copy)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Vector2D {
        let magnitude = self.magnitude();

        if magnitude == 0.0 {
            return Vector2D { x: 1.0, y: 0.0 };
        }

        Vector2D {
            x: self.x / magnitude,
            y: self.y / magnitude,
        }
    }
}
