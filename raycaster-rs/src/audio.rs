use anyhow::Result;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

pub struct Audio {
    _stream: OutputStream,
    handle: rodio::OutputStreamHandle,
    music: Option<Sink>,
}

impl Audio {
    pub fn new() -> Result<Self> {
        let (_stream, handle) = OutputStream::try_default()?;
        Ok(Self { _stream, handle, music: None })
    }

    pub fn play_music_loop(&mut self, path: &str) -> Result<()> {
        let file = File::open(path)?;
        let src = Decoder::new_looped(BufReader::new(file))?;
        let sink = Sink::try_new(&self.handle)?;
        sink.append(src);
        sink.play();
        self.music = Some(sink);
        Ok(())
    }

    pub fn play_sfx(&mut self, path: &str) -> Result<()> {
        let file = File::open(path)?;
        let src = Decoder::new(BufReader::new(file))?;
        let sink = Sink::try_new(&self.handle)?;
        sink.append(src);
        sink.detach();
        Ok(())
    }
}
