use ffmpeg_next as ffmpeg;
use ffmpeg::format::{input, Pixel};
use ffmpeg::media::Type;
use ffmpeg::software::scaling;
use ffmpeg::util::frame::video::Video;
use ffmpeg::util::frame::audio::Audio;
use std::path::Path;
use anyhow::{Context, Result};

pub struct MediaContext {
    pub input: ffmpeg::format::context::Input,
    pub video_index: Option<usize>,
    pub audio_index: Option<usize>,
    pub video_decoder: Option<ffmpeg::decoder::Video>,
    pub audio_decoder: Option<ffmpeg::decoder::Audio>,
    pub scaler: Option<scaling::context::Context>,
    pub width: u32,
    pub height: u32,
}

impl MediaContext {
    pub fn new(path: &str) -> Result<Self> {
        let input = input(&Path::new(path)).context("Falha ao abrir arquivo")?;
        
        let mut video_index = None;
        let mut audio_index = None;
        let mut video_decoder = None;
        let mut audio_decoder = None;
        let mut width = 0;
        let mut height = 0;

        // Setup Video
        if let Some(stream) = input.streams().best(Type::Video) {
            video_index = Some(stream.index());
            let context = ffmpeg::codec::context::Context::from_parameters(stream.parameters())?;
            let mut decoder = context.decoder().video()?;
            width = decoder.width();
            height = decoder.height();
            // Enable threading for decoding speed
            // decoder.set_threading(ffmpeg::threading::Config { kind: ffmpeg::threading::Type::Slice, count: 2 });
            video_decoder = Some(decoder);
        }

        // Setup Audio
        if let Some(stream) = input.streams().best(Type::Audio) {
            audio_index = Some(stream.index());
            let context = ffmpeg::codec::context::Context::from_parameters(stream.parameters())?;
            let decoder = context.decoder().audio()?;
            audio_decoder = Some(decoder);
        }

        Ok(Self {
            input,
            video_index,
            audio_index,
            video_decoder,
            audio_decoder,
            scaler: None,
            width,
            height,
        })
    }

    pub fn init_scaler(&mut self, dest_width: u32, dest_height: u32) -> Result<()> {
        if let Some(ref decoder) = self.video_decoder {
            self.scaler = Some(scaling::context::Context::get(
                decoder.format(),
                decoder.width(),
                decoder.height(),
                Pixel::RGB24,
                dest_width,
                dest_height,
                scaling::flag::Flags::BILINEAR,
            )?);
        }
        Ok(())
    }
}
