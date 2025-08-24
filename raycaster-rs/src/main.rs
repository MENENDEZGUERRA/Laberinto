mod app;
mod audio;
mod input;
mod player;
mod render;
mod ui;
mod world;
mod sprite;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    app::run()
}
