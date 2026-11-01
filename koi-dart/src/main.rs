//! Draws the profile koi in dart mode every night, whatever the recent activity.

use std::{error::Error, fs, path::Path};

use chrono::Utc;
use git_koi::behavior::Behavior;
use git_koi::coat::KoiBody;
use git_koi::collect::{GhCli, collect_account_snapshot};
use git_koi::koi::derive_koi_development;
use git_koi::motion::frame_count;
use git_koi::output::{encode_gif, encode_png};
use git_koi::render::{KoiFrame, render_koi, render_swim_frame};

const LOGIN: &str = "axellstr";
const OUTPUT_DIRECTORY: &str = "koi";

fn main() -> Result<(), Box<dyn Error>> {
    let account_snapshot = collect_account_snapshot(&GhCli, LOGIN, Utc::now().date_naive())?;
    let koi_development = derive_koi_development(&account_snapshot)?;
    let koi_body = KoiBody::Asagi;
    let behavior = Behavior::Dart;
    let swim_frames: Vec<KoiFrame> = (0..frame_count(behavior))
        .map(|frame_index| render_swim_frame(&koi_development, koi_body, behavior, frame_index))
        .collect();
    fs::create_dir_all(OUTPUT_DIRECTORY)?;
    let output_directory = Path::new(OUTPUT_DIRECTORY);
    fs::write(output_directory.join("koi.gif"), encode_gif(&swim_frames)?)?;
    fs::write(
        output_directory.join("koi.png"),
        encode_png(&render_koi(&koi_development, koi_body))?,
    )?;
    Ok(())
}
