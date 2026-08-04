use std::{
    io::Cursor,
    path::{Path, PathBuf},
    time::Duration,
};

use image::{DynamicImage, ImageReader};
use tokio::process::Command;

use crate::{
    execute::{execute_quiet, execute_to_vec},
    input::MediaInput,
    result::CassetteError,
    types::AudioMode,
};

/// Retrieves a video frame at the specified time
/// and returns it as an [`image::DynamicImage`]
pub async fn extract_frame(
    time: Duration,
    input: &MediaInput,
) -> Result<DynamicImage, CassetteError> {
    let mut command = Command::new("ffmpeg");
    command.args(["-ss", &time.as_secs().to_string()]);

    command.arg("-i");
    command.arg(&input.path);

    command.args(["-f", "image2pipe"]);
    command.args(["-vcodec", "mjpeg"]);
    command.arg("pipe:1");

    let buf = execute_to_vec(command).await?;

    let image = ImageReader::new(Cursor::new(buf));
    let decoded = image
        .with_guessed_format()
        .map_err(|_| CassetteError::UnsupportedFormat)?
        .decode()
        .map_err(|_| CassetteError::ParseOutputError)?;

    Ok(decoded)
}

pub struct FragmentParams {
    /// Indicates the start time of the fragment.
    pub start: Duration,
    /// Specifies the duration of the video fragment
    pub duration: Duration,
}

pub struct ExtractVideoFragmentParams {
    pub fragment: FragmentParams,
    /// Specifies the number of frames per second for the fragment
    pub frame_rate: Option<u8>,
    /// If True, disables the audio track in the frag
    pub audio: AudioMode,
    /// The resolution to which fragment will be scaled.
    /// If None, the original resolution will be used
    pub output_resolution: Option<(u32, u32)>,
}

/// Retrieves a video segment with the specified parameters
pub async fn extract_video_fragment(
    input: &MediaInput,
    output: impl AsRef<Path>,
    params: ExtractVideoFragmentParams,
) -> Result<(), CassetteError> {
    let mut command = Command::new("ffmpeg");

    let output = output.as_ref();
    if output.exists() {
        return Err(CassetteError::FileAlreadyExists);
    }

    command.arg("-ss");
    command.arg(params.fragment.start.as_secs().to_string());

    command.arg("-t");
    command.arg(params.fragment.duration.as_secs().to_string());

    command.arg("-i");
    command.arg(&input.path);

    if let Some(framerate) = params.frame_rate {
        command.arg("-r");
        command.arg(framerate.to_string());
    }

    match params.audio {
        AudioMode::Disabled => {
            command.arg("-an");
        }
        AudioMode::Auto => {}
    }

    if let Some((w, h)) = params.output_resolution {
        let resize = format!("\"scale=w={w}:h={h}:force_original_aspect_ratio=decrease\"");
        command.args(["-vf", &resize]);
    }

    command.arg(output);

    execute_quiet(command).await?;
    Ok(())
}

pub struct HlsPlaylistData {
    pub path: PathBuf,
}

pub struct HlsGenerationParameters {
    /// The resolution to which HLS will be scaled.
    /// If None, the original resolution will be used
    pub resize_to: Option<(u32, u32)>,

    /// Video processing mode for HLS generation
    pub mode: HlsMode,

    /// Duration of a single HLS segment
    pub segment_time: Duration,
}

/// Video processing mode for HLS generation
pub enum HlsMode {
    /// Automatic mode, where parameters are selected by `ffmpeg`
    Auto,
    /// A mode where parameters are copied from the original video
    ///
    /// It often proves to be the fastest
    Copy,
    /// Full video transcoding mode
    ///
    /// It can be useful if HLS is generated from a non-standard format
    Transcode { preset: TranscoderPreset },
}

/// Video transcoding preset
pub enum TranscoderPreset {
    Fast,
    Veryfast,
    Ultrafast,
}

impl TranscoderPreset {
    pub fn as_str(&self) -> &'static str {
        match self {
            TranscoderPreset::Fast => "fase",
            TranscoderPreset::Veryfast => "veryfast",
            TranscoderPreset::Ultrafast => "ultrafast",
        }
    }
}

/// Generates an HLS playlist for the specified video
///
/// ### Example:
/// ``` no_run
/// use cassette::{
///    ffmpeg::{self, HlsGenerationParameters, HlsMode},
///    input::MediaInput,
/// };
///
/// let input = MediaInput::new("video.mp4");
///
/// ffmpeg::generate_hls(
///     &input,
///     Path::new("playlist/index.m3u8"),
///     HlsGenerationParameters {
///         resize_to: None,
///         mode: HlsMode::Copy,
///         segment_time: Duration::from_secs(10),
///     },
/// )
/// .await
/// .unwrap();
/// ```
pub async fn generate_hls(
    input: &MediaInput,
    playlist_path: &Path,
    params: HlsGenerationParameters,
) -> Result<HlsPlaylistData, CassetteError> {
    let mut command = Command::new("ffmpeg");
    command.arg("-i");
    command.arg(&input.path);

    match params.mode {
        HlsMode::Auto => {}
        HlsMode::Copy => {
            command.args(["-codec:", "copy"]);
        }
        HlsMode::Transcode { preset } => {
            command.args(["-c:v", "libx264"]);
            command.args(["-c:a", "aac", "-ar", "44100"]);

            command.args(["-crf", "25"]);

            let preset = preset.as_str();
            command.args(["-preset", preset]);
        }
    }

    if let Some((w, h)) = params.resize_to {
        let resize = format!("scale=w={w}:h={h}");
        command.args(["-filter:v:0", &resize]);
    }

    command.args(["-hls_time", &params.segment_time.as_secs().to_string()]);
    command.args(["-hls_list_size", "0"]);
    command.args(["-hls_playlist_type", "vod"]);

    command.args(["-f", "hls"]);
    command.arg(playlist_path);

    execute_quiet(command).await?;

    Ok(HlsPlaylistData {
        path: playlist_path.to_path_buf(),
    })
}
