use anyhow::Result;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent, VirtualKeyCode, ElementState, DeviceEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const WIDTH: u32 = 320;   // render interno (escalado)
const HEIGHT: u32 = 200;

pub fn run() -> Result<()> {
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("Raycaster RS")
        .with_inner_size(LogicalSize::new(WIDTH * 3, HEIGHT * 3))
        .build(&event_loop)?;

    let surface_texture = SurfaceTexture::new(
        window.inner_size().width,
        window.inner_size().height,
        &window
    );
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;

    // TODO: inicializar mundo, jugador, audio, etc.
    // let mut app_state = AppState::new(...);

    event_loop.run(move |event, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { .. } => {
                    // TODO: input::handle_keyboard(...)
                }
                WindowEvent::Resized(size) => {
                    pixels.resize_surface(size.width, size.height).ok();
                }
                _ => {}
            },
            Event::MainEventsCleared => {
                // TODO: app_state.update(dt)
                // Render al frame buffer
                let frame = pixels.frame_mut();
                // TODO: render::draw(frame, &app_state);
                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        }
    })?;
}
