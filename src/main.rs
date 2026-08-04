use std::time::Duration;

use cassette::{
    ExtractVideoFragmentParams, FragmentParams, extract_video_fragment, input::MediaInput,
    probe_video, types::AudioMode,
};

#[tokio::main]
async fn main() {
    let input = MediaInput::try_new("assets/2.mp4").unwrap();

    dbg!(probe_video(&input).await.unwrap());
    extract_video_fragment(
        &input,
        "assets/preview.mp4",
        ExtractVideoFragmentParams {
            fragment: FragmentParams {
                start: Duration::from_secs(25),
                duration: Duration::from_secs(10),
            },
            frame_rate: Some(25),
            audio: AudioMode::Disabled,
            output_resolution: None,
        },
    )
    .await
    .unwrap();
}
