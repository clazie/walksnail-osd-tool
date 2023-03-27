use std::path::PathBuf;

use derivative::Derivative;

use super::{error::SrtFileError, frame::SrtFrame, SrtFrameData};

#[derive(Derivative)]
#[derivative(Debug)]
pub struct SrtFile {
    pub file_path: PathBuf,
    pub has_distance: bool,
    #[derivative(Debug = "ignore")]
    pub frames: Vec<SrtFrame>,
}

impl SrtFile {
    #[tracing::instrument(ret, err)]
    pub fn open(path: PathBuf) -> Result<Self, SrtFileError> {
        let mut has_distance = false;
        let srt_frames = srtparse::from_file(&path)?
            .iter()
            .map(|i| -> Result<SrtFrame, SrtFileError> {
                let data: SrtFrameData = i.text.parse()?;
                has_distance |= data.distance > 0;
                Ok(SrtFrame {
                    start_time_secs: i.start_time.into_duration().as_secs_f32(),
                    end_time_secs: i.end_time.into_duration().as_secs_f32(),
                    data,
                })
            })
            .collect::<Result<_, _>>()?;

        Ok(Self {
            file_path: path,
            has_distance,
            frames: srt_frames,
        })
    }
}
