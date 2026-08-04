use crate::{ffprobe::StreamDataJson, result::CassetteError};

#[derive(Debug)]
pub struct VideoStreamMetadata {
    pub codec: VideoCodec,
    pub resolution: Resolution,
    pub frame_rate: String,
    pub duration_secs: f32,
}

impl TryFrom<StreamDataJson> for VideoStreamMetadata {
    type Error = CassetteError;
    fn try_from(json: StreamDataJson) -> Result<Self, Self::Error> {
        let meta = VideoStreamMetadata {
            codec: json
                .codec_name
                .ok_or(CassetteError::ParseOutputError)?
                .into(),
            resolution: Resolution {
                width: json.width.ok_or(CassetteError::ParseOutputError)?,
                height: json.height.ok_or(CassetteError::ParseOutputError)?,
            },
            frame_rate: json.r_frame_rate.ok_or(CassetteError::ParseOutputError)?,
            duration_secs: json
                .duration
                .parse()
                .map_err(|_| CassetteError::ParseOutputError)?,
        };
        Ok(meta)
    }
}

#[derive(Debug)]
pub struct AudioStreamMetadata {
    pub codec: AudioCodec,
    pub duration_secs: f32,
}

impl TryFrom<StreamDataJson> for AudioStreamMetadata {
    type Error = CassetteError;
    fn try_from(json: StreamDataJson) -> Result<Self, Self::Error> {
        let meta = AudioStreamMetadata {
            codec: json
                .codec_name
                .ok_or(CassetteError::ParseOutputError)?
                .into(),
            duration_secs: json
                .duration
                .parse()
                .map_err(|_| CassetteError::ParseOutputError)?,
        };
        Ok(meta)
    }
}

#[derive(Debug)]
pub struct VideoMetadata {
    pub video: VideoStreamMetadata,
    pub audio: Option<AudioStreamMetadata>,
}

#[derive(Debug)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, PartialEq)]
pub enum VideoCodec {
    H264,
    H265,
    Av1,
    Other(String),
}

impl From<String> for VideoCodec {
    fn from(c: String) -> Self {
        match c.as_str() {
            "h264" => VideoCodec::H264,
            "h265" => VideoCodec::H265,
            "av1" => VideoCodec::Av1,
            _ => VideoCodec::Other(c),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum AudioCodec {
    Aac,
    Mp3,
    Opus,
    Flac,
    Other(String),
}

impl From<String> for AudioCodec {
    fn from(c: String) -> Self {
        match c.as_str() {
            "aac" => AudioCodec::Aac,
            "mp3" => AudioCodec::Mp3,
            "opus" => AudioCodec::Opus,
            "flac" => AudioCodec::Flac,
            _ => AudioCodec::Other(c),
        }
    }
}

pub enum AudioMode {
    Disabled,
    Auto,
}
