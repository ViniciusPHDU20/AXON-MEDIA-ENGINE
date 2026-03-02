use ffmpeg_next as ffmpeg;
use ffmpeg::format::context::Input;
use ffmpeg::codec::context::Context;
use ffmpeg::decoder::Video;
use anyhow::{Context as AnyhowContext, Result};

pub struct VideoDecoder {
    pub input: Input,
    pub decoder: Video,
    pub video_stream_index: usize,
}

impl VideoDecoder {
    pub fn new(file_path: &str) -> Result<Self> {
        let input = ffmpeg::format::input(&file_path)
            .map_err(|e| anyhow::anyhow!("Erro ao abrir arquivo: {}", e))?;

        let stream = input
            .streams()
            .best(ffmpeg::media::Type::Video)
            .ok_or_else(|| anyhow::anyhow!("Nenhuma trilha de vídeo encontrada"))?;

        let video_stream_index = stream.index();
        let context_decoder = Context::from_parameters(stream.parameters())?;
        let decoder = context_decoder.decoder().video()?;

        Ok(Self {
            input,
            decoder,
            video_stream_index,
        })
    }

    pub fn width(&self) -> u32 {
        self.decoder.width()
    }

    pub fn height(&self) -> u32 {
        self.decoder.height()
    }
}
