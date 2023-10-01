mod engine;
mod time;
mod utils;

use anyhow::{Context, Ok};
use clap::Parser;
use hyprland::event_listener::EventListener;
use hyprland::shared::WorkspaceType;
use rand::{seq::SliceRandom, thread_rng};
use std::path::PathBuf;
use which::which;

use crate::engine::{get_engine, Engine};
use crate::time::enough_time_between_changes;
use crate::utils::{get_valid_image_paths_from_provided_dir, CyclerError};

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Application {
    Swww,
    Hyprpaper,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long)]
    pub backgrounds_path: String,
    #[arg(long)]
    #[clap(value_enum, default_value_t=Application::Swww)]
    pub binary: Application,
    #[arg(long)]
    pub minutes: Option<i32>,
}

pub fn handle_workspace_change(
    data: WorkspaceType,
    binary: &Box<dyn Engine>,
    valid_image_paths: &Vec<PathBuf>,
    minutes: &i32,
) -> anyhow::Result<()> {
    if !enough_time_between_changes(&minutes) {
        return Ok(());
    }

    println!("going into matching workspacetype");

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

            println!("path of chosen file: {}", path_of_chosen_file);

            binary
                .change(path_of_chosen_file)
                .context("Failed to send")?;

            Ok(())
        }
        WorkspaceType::Special(_) => Ok(()),
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let binary: Box<dyn Engine> = get_engine(args.binary);

    which(binary.which())
        .with_context(|| format!("'{}' binary not found on PATH", binary.which()))?;

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
        handle_workspace_change(data, &binary, &image_file_paths, &minutes_between_change).ok();
    });

    event_listener
        .start_listener()
        .context("Failed to start listener")?;

    Ok(())
}
