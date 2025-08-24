use glam::{Mat2, Vec2};
use crate::input::InputState;
use crate::world::World;

pub struct Player {
    pub pos: Vec2,
    pub dir: Vec2,   // PARA ADELANTE ( SE PSUPONE)
    pub plane: Vec2, // FOV
    pub speed_move: f32,
    pub speed_rot: f32,
    pub radius: f32,
}

impl Player {
    pub fn from_world_spawn(world: &World) -> Self {
        let dir_rad = world.player_dir_deg.to_radians();
        let dir = Vec2::new(dir_rad.cos(), dir_rad.sin());
        let plane = Vec2::new(-dir.y, dir.x) * 0.90; // ~FOV 66° clásico
        Self {
            pos: world.player_spawn,
            dir,
            plane,
            speed_move: 3.0,
            speed_rot: 2.5,
            radius: 0.2,
        }
    }

    pub fn update(&mut self, world: &World, input: &mut InputState, dt: f32) {
        // Giro
        let mut rot = 0.0;
        if input.rotate_left {
            rot -= 1.0;
        }
        if input.rotate_right {
            rot += 1.0;
        }
        rot += input.mouse_dx * 0.003; // sensibilidad
        if rot.abs() > 0.0 {
            self.rotate(rot * self.speed_rot * dt);
        }

        // Movimiento
        let mut move_dir = Vec2::ZERO;
        if input.forward {
            move_dir += self.dir;
        }
        if input.back {
            move_dir -= self.dir;
        }
        if input.strafe_left {
            move_dir += Vec2::new(-self.dir.y, -self.dir.x);
        }
        if input.strafe_right {
            move_dir += Vec2::new(self.dir.y, self.dir.x);
        }
        if move_dir.length_squared() > 0.0 {
            move_dir = move_dir.normalize();
        }
        let step = move_dir * self.speed_move * dt;

        // Colisiones
        let next_x = self.pos + Vec2::new(step.x, 0.0);
        if !collides(world, next_x, self.radius) {
            self.pos = next_x;
        }
        let next_y = self.pos + Vec2::new(0.0, step.y);
        if !collides(world, next_y, self.radius) {
            self.pos = next_y;
        }
    }

    fn rotate(&mut self, angle: f32) {
        let rot = Mat2::from_angle(angle);
        self.dir = rot * self.dir;
        self.plane = rot * self.plane;
    }
}

fn collides(world: &World, pos: Vec2, r: f32) -> bool {
    let min_x = (pos.x - r).floor() as i32;
    let max_x = (pos.x + r).floor() as i32;
    let min_y = (pos.y - r).floor() as i32;
    let max_y = (pos.y + r).floor() as i32;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if world.is_wall(x, y) {
                return true;
            }
        }
    }
    false
}
