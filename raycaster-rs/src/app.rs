use anyhow::Result;
use log::{error, info};
use pixels::{Pixels, SurfaceTexture};
use std::time::{Duration, Instant};
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use crate::audio::Audio;
use crate::input::{handle_keyboard_input, handle_mouse_move, InputState};
use crate::player::Player;
use crate::render::{draw_frame, HEIGHT, WIDTH};
use crate::ui::{GameMode, UiState};
use crate::world::World;

pub fn run() -> Result<()> {
    // Ventana y framebuffer
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Raycaster RS")
        .with_inner_size(LogicalSize::new((WIDTH * 3) as f64, (HEIGHT * 3) as f64))
        .build(&event_loop)
        .expect("No se pudo crear ventana");

    let size = window.inner_size();
    let surface_texture = SurfaceTexture::new(size.width, size.height, &window);
    let mut pixels = Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?;

    // Estado del juego
    let mut input = InputState::default();
    let mut ui = UiState::default();
    let mut world = World::from_file("assets/levels/level1.json").unwrap_or_else(|e| {
        info!("No se pudo cargar level1.json ({e:?}), usando nivel por defecto");
        World::default_level()
    });
    let mut player = Player::from_world_spawn(&world);

    // Audio 
    let mut audio = Audio::new().ok();
    if let Some(a) = &mut audio {
        let _ = a.play_music_loop("assets/audio/bgm.ogg"); // NOTA FUTURO YO: IMPLEMENTA ESTO (AUN NO ENCUENTRO EL LOOP)
    }

    // Loop temporal
    let mut last = Instant::now();
    let mut fps_timer = Instant::now();
    let mut frames: u32 = 0;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(size) => {
                    if let Err(e) = pixels.resize_surface(size.width, size.height) {
                        error!("resize_surface error: {e:?}");
                    }
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    if let Err(e) =
                        pixels.resize_surface(new_inner_size.width, new_inner_size.height)
                    {
                        error!("scale_factor_changed error: {e:?}");
                    }
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(key),
                            state,
                            ..
                        },
                    ..
                } => {
                    // UI y entrada
                    if ui.mode == GameMode::Welcome {
                        if state == ElementState::Pressed && key == VirtualKeyCode::Return {
                            ui.mode = GameMode::Playing;
                        }
                    } else if ui.mode == GameMode::Success {
                        if state == ElementState::Pressed && key == VirtualKeyCode::Return {
                            // Reiniciar nivel
                            player = Player::from_world_spawn(&world);
                            ui.mode = GameMode::Playing;
                        }
                    } else {
                        // Playing
                        handle_keyboard_input(&mut input, key, state);

                        if state == ElementState::Pressed {
                            match key {
                                VirtualKeyCode::Escape => {
                                    // Toggle captura de mouse
                                    input.mouse_captured = !input.mouse_captured;
                                    let _ = window.set_cursor_visible(!input.mouse_captured);
                                }
                                VirtualKeyCode::R => {
                                    player = Player::from_world_spawn(&world);
                                }
                                VirtualKeyCode::Key1 => {
                                    if let Ok(w) = World::from_file("assets/levels/level1.json") {
                                        world = w;
                                        player = Player::from_world_spawn(&world);
                                    }
                                }
                                VirtualKeyCode::Key2 => {
                                    if let Ok(w) = World::from_file("assets/levels/level2.json") {
                                        world = w;
                                        player = Player::from_world_spawn(&world);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    handle_mouse_move(&mut input, position);
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                // dt
                let now = Instant::now();
                let mut dt = now.duration_since(last).as_secs_f32();
                last = now;
                if dt > 0.1 {
                    dt = 0.1;
                } // clamp anti-saltos

                // Update
                if ui.mode == GameMode::Playing {
                    player.update(&world, &mut input, dt);

                    // Condición de éxito
                    if world.is_on_exit_tile(player.pos.x, player.pos.y) {
                        ui.mode = GameMode::Success;
                        if let Some(a) = &mut audio {
                            let _ = a.play_sfx("assets/audio/step.wav");
                        }
                    }
                }

                // Render
                let frame = pixels.frame_mut();
                draw_frame(frame, &world, &player, &ui, &mut input);

                if let Err(e) = pixels.render() {
                    error!("pixels.render error: {e:?}");
                    *control_flow = ControlFlow::Exit;
                }

                // FPS
                frames += 1;
                if fps_timer.elapsed() >= Duration::from_millis(500) {
                    ui.fps = (frames as f32) / (fps_timer.elapsed().as_secs_f32());
                    frames = 0;
                    fps_timer = Instant::now();
                }

                // limpiar deltas de mouse por frame
                input.mouse_dx = 0.0;
            }
            _ => {}
        }
    });
}
