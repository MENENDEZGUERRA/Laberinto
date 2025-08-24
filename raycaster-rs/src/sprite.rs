use glam::Vec2;

pub struct Sprite {
    pub pos: Vec2,
    pub color_a: u32,
    pub color_b: u32,
    pub t: f32,
    pub period: f32,
}

impl Sprite {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            pos: Vec2::new(x, y),
            color_a: 0xffaa00ff,
            color_b: 0xffff00ff,
            t: 0.0,
            period: 0.5,
        }
    }
    pub fn update(&mut self, dt: f32) {
        self.t = (self.t + dt) % (2.0 * self.period);
    }
    pub fn color(&self) -> u32 {
        if self.t < self.period {
            self.color_a
        } else {
            self.color_b
        }
    }
}
