mod backends;
mod time;
mod utils;

use anyhow::{Context, Ok};
use clap::Parser;
use hyprland::event_listener::EventListener;
use hyprland::shared::WorkspaceType;
use rand::{seq::SliceRandom, thread_rng};
use std::{path::PathBuf, process::Command};
use which::which;

use crate::time::should_change;
use crate::utils::{get_valid_image_paths_from_provided_dir, CyclerError};

const SWWW_BINARY: &str = "swww";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    pub backgrounds_path: String,
    #[arg(long)]
    pub minutes: Option<i32>,
}

pub fn handle_workspace_change(
    data: WorkspaceType,
    valid_image_paths: &Vec<PathBuf>,
    minutes: &i32,
) -> anyhow::Result<()> {
    match data {
        WorkspaceType::Regular(_) => {
            let mut rng = thread_rng();
            let chosen_file = valid_image_paths
                .choose(&mut rng)
                .ok_or(CyclerError::CantChooseAnImage)
                .context("Couldn't select a file from array of valid images. Exiting")?;

            let path_of_chosen_file = chosen_file
                .as_path()
                .to_str()
                .ok_or(CyclerError::CantConvertToStr)
                .with_context(|| format!("Failed to convert {:?} to &str", chosen_file))?;

            if !should_change(&minutes) {
                return Ok(());
            }

            Command::new("swww")
                .args(["img", path_of_chosen_file])
                .output()
                .context("Failed to issue 'swww' command")?;

            Ok(())
        }
        WorkspaceType::Special(_) => Ok(()),
    }
}

fn main() -> anyhow::Result<()> {
    which(SWWW_BINARY).with_context(|| format!("'{}' binary not found on PATH", SWWW_BINARY))?;

    let args = Args::parse();
    PathBuf::from(&args.backgrounds_path)
        .is_dir()
        .then_some(true)
        .ok_or(CyclerError::DirectoryNotFound)
        .with_context(|| format!("Directory {} not found", &args.backgrounds_path))?;

    let minutes_between_change = args.minutes.unwrap_or(5);

    let image_file_paths = get_valid_image_paths_from_provided_dir(&args.backgrounds_path)
        .with_context(|| format!("No images found in {}", &args.backgrounds_path))?;

    let mut event_listener = EventListener::new();
    event_listener.add_workspace_change_handler(move |data| {
        handle_workspace_change(data, &image_file_paths, &minutes_between_change).ok();
    });

    event_listener
        .start_listener()
        .context("Failed to start listener")?;

    Ok(())
}
