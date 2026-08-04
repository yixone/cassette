use serde::Deserialize;
use tokio::process::Command;

use crate::{
    execute::execute_to_vec,
    input::MediaInput,
    result::CassetteError,
    types::{AudioStreamMetadata, VideoMetadata, VideoStreamMetadata},
};

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct StreamDataJson {
    #[serde(default)]
    pub codec_name: Option<String>,
    pub codec_type: String,
    #[serde(default)]
    pub width: Option<u32>,
    #[serde(default)]
    pub height: Option<u32>,
    #[serde(default)]
    pub r_frame_rate: Option<String>,
    pub duration: String,
}

#[derive(Debug, Deserialize)]
struct ProbeDataJson {
    streams: Vec<StreamDataJson>,
}

/// Reads video stream metadata from a file and returns it as [`VideoStreamData`]
pub async fn probe_video(input: &MediaInput) -> Result<VideoMetadata, CassetteError> {
    let mut command = Command::new("ffprobe");
    command.arg("-hide_banner");
    command.args(["-v", "error"]);
    command.args(["-show_streams"]);
    command.args([
        "-show_entries",
        "stream=codec_name,codec_type,width,height,r_frame_rate,duration",
    ]);
    command.args(["-of", "json"]);
    command.arg("-i");
    command.arg(&input.path);

    let res = execute_to_vec(command).await?;
    let data: ProbeDataJson =
        serde_json::from_slice(&res).map_err(|_| CassetteError::ParseOutputError)?;

    let Some(video_meta_json) = data
        .streams
        .iter()
        .find(|s| &s.codec_type == "video")
        .cloned()
    else {
        return Err(CassetteError::UnsupportedFormat);
    };
    let video = VideoStreamMetadata::try_from(video_meta_json)?;

    let audio = if let Some(audio_meta_json) = data
        .streams
        .iter()
        .find(|s| &s.codec_type == "audio")
        .cloned()
    {
        Some(AudioStreamMetadata::try_from(audio_meta_json)?)
    } else {
        None
    };

    let meta = VideoMetadata { video, audio };
    Ok(meta)
}
