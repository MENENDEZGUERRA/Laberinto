use anyhow::{anyhow, Result};
use glam::{IVec2, Vec2};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

pub struct Texture {
    pub w: u32,
    pub h: u32,
    pub pixels: Vec<u32>, // RGBA8 empaquetado (little-endian)
}

impl Texture {
    pub fn from_file(path: &str) -> Result<Self> {
        let img = image::open(path)?.to_rgba8();
        let (w, h) = (img.width(), img.height());
        let raw = img.into_raw(); // Vec<u8> RGBA
        let pixels = raw
            .chunks_exact(4)
            .map(|px| u32::from_le_bytes([px[0], px[1], px[2], px[3]]))
            .collect();
        Ok(Self { w, h, pixels })
    }
}

#[derive(Clone)]
pub struct TileMap {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<u8>, 
}

impl TileMap {
    #[inline]
    pub fn idx(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return None;
        }
        Some((y * self.width + x) as usize)
    }

    #[inline]
    pub fn tile(&self, x: i32, y: i32) -> u8 {
        // NOTA: fuera del mapa = pared (id 1)
        self.idx(x, y).map(|i| self.tiles[i]).unwrap_or(1)
    }
}

pub struct World {
    pub map: TileMap,
    pub textures: HashMap<u8, Texture>,
    pub player_spawn: Vec2,
    pub player_dir_deg: f32,
    pub exit_tile: Option<IVec2>,
    pub name: String,
}

impl World {
    pub fn default_level() -> Self {
        let raw = [
            "1111111111111111",
            "1000000000000001",
            "1022200003333001",
            "1000000000000001",
            "1000000000000001",
            "1000044440000001",
            "1000000000000001",
            "11111111111111E1",
        ];
        let height = raw.len() as i32;
        let width = raw[0].len() as i32;
        let mut tiles = Vec::with_capacity((width * height) as usize);
        let mut exit = None;
        for (y, row) in raw.iter().enumerate() {
            for (x, ch) in row.chars().enumerate() {
                match ch {
                    'E' => {
                        tiles.push(0);
                        exit = Some(IVec2::new(x as i32, y as i32));
                    }
                    '0' => tiles.push(0),
                    c @ '1'..='9' => tiles.push(c as u8 - b'0'),
                    _ => tiles.push(1),
                }
            }
        }
        Self {
            map: TileMap { width, height, tiles },
            textures: HashMap::new(),
            player_spawn: Vec2::new(2.5, 2.5),
            player_dir_deg: 0.0,
            exit_tile: exit,
            name: "default".to_string(),
        }
    }

    pub fn from_file(path: &str) -> Result<Self> {
        let s = fs::read_to_string(path)?;
        let lvl: LevelFile = serde_json::from_str(&s)?;
        let width = lvl.width as i32;
        let height = lvl.height as i32;
        if lvl.tiles.len() as i32 != height {
            return Err(anyhow!("tiles rows != height"));
        }
        let mut tiles = Vec::with_capacity((width * height) as usize);
        let mut exit = None;
        for (y, row) in lvl.tiles.iter().enumerate() {
            if row.len() as i32 != width {
                return Err(anyhow!("row {y} width mismatch"));
            }
            for (x, ch) in row.chars().enumerate() {
                match ch {
                    'E' => {
                        tiles.push(0);
                        exit = Some(IVec2::new(x as i32, y as i32));
                    }
                    '0' => tiles.push(0),
                    c @ '1'..='9' => tiles.push(c as u8 - b'0'),
                    _ => tiles.push(1),
                }
            }
        }

        // Cargar TODAS las texturas
        let mut textures = HashMap::new();
        if let Some(map) = lvl.textures {
            for (k, v) in map {
                if let Ok(id) = k.parse::<u8>() {
                    if let Ok(tex) = Texture::from_file(&v) {
                        textures.insert(id, tex);
                    } else {
                        eprintln!("WARN: no se pudo cargar textura id {} desde '{}'", id, v);
                    }
                } else {
                    eprintln!("WARN: id de textura inválido: '{}'", k);
                }
            }
        }

        Ok(Self {
            map: TileMap { width, height, tiles },
            textures,
            player_spawn: Vec2::new(lvl.player_start.x, lvl.player_start.y),
            player_dir_deg: lvl.player_start.dir_deg,
            exit_tile: lvl.exit.tile.map(|t| IVec2::new(t.x, t.y)).or(exit),
            name: lvl.name.unwrap_or_else(|| "level".into()),
        })
    }

    pub fn is_wall(&self, x: i32, y: i32) -> bool {
        self.map.tile(x, y) > 0
    }

    pub fn tile_id(&self, x: i32, y: i32) -> u8 {
        self.map.tile(x, y)
    }

    pub fn is_on_exit_tile(&self, x: f32, y: f32) -> bool {
        if let Some(t) = self.exit_tile {
            (x as i32, y as i32) == (t.x, t.y)
        } else {
            false
        }
    }
}

#[derive(Deserialize)]
struct LevelFile {
    pub name: Option<String>,
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<String>,
    pub player_start: PlayerStart,
    pub textures: Option<HashMap<String, String>>, // id -> ruta
    #[serde(default)]
    pub exit: ExitSpec,
}

#[derive(Deserialize)]
struct PlayerStart {
    pub x: f32,
    pub y: f32,
    pub dir_deg: f32,
}

#[derive(Deserialize, Default)]
struct ExitSpec {
    pub tile: Option<TilePos>,
}

#[derive(Deserialize)]
struct TilePos {
    pub x: i32,
    pub y: i32,
}
