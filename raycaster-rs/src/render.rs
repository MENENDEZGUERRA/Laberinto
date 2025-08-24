use crate::input::InputState;
use crate::player::Player;
use crate::ui::{GameMode, UiState};
use crate::world::World;
use glam::Vec2;

pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 200;

pub fn draw_frame(
    frame: &mut [u8],
    world: &World,
    player: &Player,
    ui: &UiState,
    _input: &mut InputState,
) {
    // Fondo
    clear_color(frame, 0x202028ff);

    match ui.mode {
        GameMode::Welcome => {
            draw_centered_text(frame, "RAYCASTER RS", 20);
            draw_text(frame, 40, 70, "ENTER para jugar");
            draw_text(frame, 40, 85, "WASD: mover, Flechas: rotar");
            draw_text(frame, 40, 100, "ESC: capturar/soltar mouse");
            draw_text(frame, 40, 115, "1/2: cambiar nivel");
        }
        GameMode::Playing | GameMode::Success => {
            // Suelo y cielo
            draw_hsplit(frame, 0, HEIGHT as i32 / 2, 0x303040ff);
            draw_hsplit(frame, HEIGHT as i32 / 2, HEIGHT as i32, 0x0f0f10ff);

            // Ray casting paredes
            let mut zbuf = [0.0f32; WIDTH];
            cast_walls(frame, world, player, &mut zbuf);

            // Minimap
            draw_minimap(frame, world, player);

            // HUD
            draw_text(frame, 4, 4, &format!("FPS:{}", ui.fps as i32));

            if ui.mode == GameMode::Success {
                draw_centered_text(frame, "¡Nivel completado!", HEIGHT as i32 / 2 - 10);
                draw_centered_text(frame, "ENTER para reiniciar", HEIGHT as i32 / 2 + 6);
            }
        }
    }
}

fn clear_color(frame: &mut [u8], color: u32) {
    for px in frame.chunks_exact_mut(4) {
        px.copy_from_slice(&color.to_le_bytes());
    }
}

fn draw_hsplit(frame: &mut [u8], y0: i32, y1: i32, color: u32) {
    let y0 = y0.clamp(0, HEIGHT as i32);
    let y1 = y1.clamp(0, HEIGHT as i32);
    for y in y0..y1 {
        let row = y as usize * WIDTH * 4;
        for x in 0..WIDTH {
            let i = row + x * 4;
            frame[i..i + 4].copy_from_slice(&color.to_le_bytes());
        }
    }
}

fn cast_walls(frame: &mut [u8], world: &World, player: &Player, zbuf: &mut [f32; WIDTH]) {
    for x in 0..WIDTH {
        let camera_x = 2.0 * (x as f32) / (WIDTH as f32) - 1.0;
        let ray_dir = player.dir + player.plane * camera_x;

        let mut map_x = player.pos.x.floor() as i32;
        let mut map_y = player.pos.y.floor() as i32;

        let delta_dist = Vec2::new(
            if ray_dir.x == 0.0 { 1e30 } else { (1.0 / ray_dir.x).abs() },
            if ray_dir.y == 0.0 { 1e30 } else { (1.0 / ray_dir.y).abs() },
        );

        let (step_x, mut side_dist_x) = if ray_dir.x < 0.0 {
            (-1, (player.pos.x - map_x as f32) * delta_dist.x)
        } else {
            (1, (map_x as f32 + 1.0 - player.pos.x) * delta_dist.x)
        };
        let (step_y, mut side_dist_y) = if ray_dir.y < 0.0 {
            (-1, (player.pos.y - map_y as f32) * delta_dist.y)
        } else {
            (1, (map_y as f32 + 1.0 - player.pos.y) * delta_dist.y)
        };

        // DDA
        let mut side = 0;
        loop {
            if side_dist_x < side_dist_y {
                side_dist_x += delta_dist.x;
                map_x += step_x;
                side = 0;
            } else {
                side_dist_y += delta_dist.y;
                map_y += step_y;
                side = 1;
            }
            if world.is_wall(map_x, map_y) {
                break;
            }
        }

        let perp_dist = if side == 0 {
            (map_x as f32 - player.pos.x + (1 - step_x) as f32 / 2.0) / ray_dir.x
        } else {
            (map_y as f32 - player.pos.y + (1 - step_y) as f32 / 2.0) / ray_dir.y
        };
        let perp_dist = perp_dist.max(0.0001);
        zbuf[x] = perp_dist;

        let line_h = (HEIGHT as f32 / perp_dist) as i32;
        let draw_start = (-line_h / 2 + (HEIGHT as i32) / 2).clamp(0, HEIGHT as i32 - 1);
        let draw_end = (line_h / 2 + (HEIGHT as i32) / 2).clamp(0, HEIGHT as i32 - 1);

        let id = world.tile_id(map_x, map_y);
        let base = id_to_color(id);
        let shade = (1.0 / (1.0 + 0.1 * perp_dist)).min(1.0) * if side == 1 { 0.85 } else { 1.0 };
        let color = mul_color(base, shade as f32);

        for y in draw_start..=draw_end {
            let i = (y as usize * WIDTH + x) * 4;
            frame[i..i + 4].copy_from_slice(&color.to_le_bytes());
        }
    }
}

fn id_to_color(id: u8) -> u32 {
    match id {
        1 => 0xcc4444ff,
        2 => 0x44cc44ff,
        3 => 0x4444ccff,
        4 => 0xcccc44ff,
        5 => 0x44ccccff,
        6 => 0xcc44ccff,
        7 => 0x88cc44ff,
        8 => 0x44cc88ff,
        9 => 0xcc8844ff,
        _ => 0xffffffff,
    }
}

fn mul_color(c: u32, k: f32) -> u32 {
    let r = ((c & 0x000000ff) as f32 * k).clamp(0.0, 255.0) as u8;
    let g = (((c >> 8) & 0xff) as f32 * k).clamp(0.0, 255.0) as u8;
    let b = (((c >> 16) & 0xff) as f32 * k).clamp(0.0, 255.0) as u8;
    let a = ((c >> 24) & 0xff) as u8;
    u32::from_le_bytes([r, g, b, a])
}

fn draw_minimap(frame: &mut [u8], world: &World, player: &Player) {
    let scale = 4;
    let margin = 4;
    let w = (world.map.width as usize) * scale;
    let h = (world.map.height as usize) * scale;
    let origin_x = WIDTH - margin - w;
    let origin_y = margin;
    for my in 0..world.map.height {
        for mx in 0..world.map.width {
            let id = world.tile_id(mx, my);
            let color = if id == 0 {
                0x00000088
            } else {
                mul_color(id_to_color(id), 0.6)
            };
            fill_rect(
                frame,
                origin_x + (mx as usize) * scale,
                origin_y + (my as usize) * scale,
                scale,
                scale,
                color,
            );
        }
    }
    let px = origin_x as i32 + (player.pos.x as i32) * (scale as i32);
    let py = origin_y as i32 + (player.pos.y as i32) * (scale as i32);
    fill_rect(frame, px as usize, py as usize, 3, 3, 0xffffffff);
}

fn fill_rect(frame: &mut [u8], x: usize, y: usize, w: usize, h: usize, color: u32) {
    for yy in 0..h {
        let yy2 = y + yy;
        if yy2 >= HEIGHT {
            break;
        }
        let row = yy2 * WIDTH * 4;
        for xx in 0..w {
            let xx2 = x + xx;
            if xx2 >= WIDTH {
                break;
            }
            let i = row + xx2 * 4;
            frame[i..i + 4].copy_from_slice(&color.to_le_bytes());
        }
    }
}

// --- Texto bitmap muy simple ---

fn draw_centered_text(frame: &mut [u8], s: &str, y: i32) {
    let w = (s.len() as i32) * 6;
    let x = ((WIDTH as i32) - w) / 2;
    draw_text(frame, x, y, s);
}

pub fn draw_text(frame: &mut [u8], mut x: i32, y: i32, s: &str) {
    for ch in s.chars() {
        if ch == ' ' {
            x += 6;
            continue;
        }
        draw_char(frame, x, y, ch);
        x += 6;
    }
}

// ARREGLAR EFECTO ESPEJO
fn draw_char(frame: &mut [u8], x: i32, y: i32, ch: char) {
    let glyph = font5x7(ch);
    for (yy, row) in glyph.iter().enumerate() {
        for xx in 0..5 {
            if (row >> (4 - xx)) & 1 == 1 {
                let px = x + xx as i32;
                let py = y + yy as i32;
                if px >= 0 && px < WIDTH as i32 && py >= 0 && py < HEIGHT as i32 {
                    let i = (py as usize * WIDTH + px as usize) * 4;
                    frame[i..i + 4].copy_from_slice(&0xffffffffu32.to_le_bytes());
                }
            }
        }
    }
}

fn font5x7(ch: char) -> [u8; 7] {
    match ch.to_ascii_uppercase() {
        'A' => [0b01110,0b10001,0b10001,0b11111,0b10001,0b10001,0b10001],
        'C' => [0b01110,0b10001,0b10000,0b10000,0b10000,0b10001,0b01110],
        'D' => [0b11110,0b10001,0b10001,0b10001,0b10001,0b10001,0b11110],
        'E' => [0b11111,0b10000,0b11110,0b10000,0b10000,0b10000,0b11111],
        'F' => [0b11111,0b10000,0b11110,0b10000,0b10000,0b10000,0b10000],
        'G' => [0b01110,0b10001,0b10000,0b10111,0b10001,0b10001,0b01110],
        'I' => [0b11111,0b00100,0b00100,0b00100,0b00100,0b00100,0b11111],
        'J' => [0b00001,0b00001,0b00001,0b00001,0b10001,0b10001,0b01110],
        'L' => [0b10000,0b10000,0b10000,0b10000,0b10000,0b10000,0b11111],
        'M' => [0b10001,0b11011,0b10101,0b10101,0b10001,0b10001,0b10001],
        'N' => [0b10001,0b11001,0b10101,0b10011,0b10001,0b10001,0b10001],
        'O' => [0b01110,0b10001,0b10001,0b10001,0b10001,0b10001,0b01110],
        'P' => [0b11110,0b10001,0b10001,0b11110,0b10000,0b10000,0b10000],
        'R' => [0b11110,0b10001,0b10001,0b11110,0b10100,0b10010,0b10001],
        'S' => [0b01111,0b10000,0b10000,0b01110,0b00001,0b00001,0b11110],
        'T' => [0b11111,0b00100,0b00100,0b00100,0b00100,0b00100,0b00100],
        'U' => [0b10001,0b10001,0b10001,0b10001,0b10001,0b10001,0b01110],
        'V' => [0b10001,0b10001,0b10001,0b10001,0b01010,0b01010,0b00100],
        'Y' => [0b10001,0b01010,0b00100,0b00100,0b00100,0b00100,0b00100],
        '¡' | '!' => [0b00100,0b00100,0b00100,0b00100,0b00100,0b00000,0b00100],
        ':' => [0b00000,0b00100,0b00000,0b00000,0b00000,0b00100,0b00000],
        '0' => [0b01110,0b10001,0b10011,0b10101,0b11001,0b10001,0b01110],
        '1' => [0b00100,0b01100,0b00100,0b00100,0b00100,0b00100,0b01110],
        '2' => [0b01110,0b10001,0b00001,0b00110,0b01000,0b10000,0b11111],
        '3' => [0b11110,0b00001,0b00001,0b01110,0b00001,0b00001,0b11110],
        '4' => [0b00010,0b00110,0b01010,0b10010,0b11111,0b00010,0b00010],
        '5' => [0b11111,0b10000,0b11110,0b00001,0b00001,0b10001,0b01110],
        '6' => [0b00110,0b01000,0b10000,0b11110,0b10001,0b10001,0b01110],
        '7' => [0b11111,0b00001,0b00010,0b00100,0b01000,0b01000,0b01000],
        '8' => [0b01110,0b10001,0b10001,0b01110,0b10001,0b10001,0b01110],
        '9' => [0b01110,0b10001,0b10001,0b01111,0b00001,0b00010,0b01100],
        _ => [0, 0, 0, 0, 0, 0, 0],
    }
}
