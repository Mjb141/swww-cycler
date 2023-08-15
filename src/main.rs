mod utils;

use anyhow::Context;
use clap::Parser;
use env_logger::Env;
use hyprland::event_listener::EventListener;
use hyprland::shared::WorkspaceType;
use log::{debug, error};
use rand::{seq::SliceRandom, thread_rng};
use std::{path::PathBuf, process::Command};
use which::which;

use crate::utils::get_valid_image_paths_from_provided_dir;
use crate::utils::CyclerError;

const SWWW_BINARY: &str = "swww";

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long)]
    pub backgrounds_path: String,
}

pub fn handle_workspace_change(data: WorkspaceType, valid_image_paths: &Vec<PathBuf>) {
    match data {
        WorkspaceType::Regular(_) => {
            debug!(
                "WorkspaceChange event (Regular) detected, attempting to change background image"
            );
            let mut rng = thread_rng();
            let chosen_file = match valid_image_paths.choose(&mut rng) {
                Some(selected_file) => selected_file,
                None => {
                    error!("Couldn't select a file from array of valid images. Exiting");
                    panic!()
                }
            };

            let path_of_chosen_file = match chosen_file.as_path().to_str() {
                Some(path_of_image) => path_of_image,
                None => {
                    error!("Couldn't convert path of selected file. Exiting");
                    panic!()
                }
            };

            if let Err(_) = Command::new("swww")
                .args(["img", path_of_chosen_file])
                .output()
            {
                error!("Failed to issue 'swww' command. Not changing background")
            }
        }
        WorkspaceType::Special(_) => {
            debug!("WorkspaceChange event (Special) ignored");
        }
    }
}

fn main() -> anyhow::Result<()> {
    let env = Env::default().filter_or("SWWW_CYCLER_LOG_LEVEL", "info");
    env_logger::init_from_env(env);

    which(SWWW_BINARY).with_context(|| format!("'{}' binary not found on PATH", SWWW_BINARY))?;

    let args = Args::parse();
    PathBuf::from(&args.backgrounds_path)
        .is_dir()
        .then_some(true)
        .ok_or(CyclerError::DirectoryNotFound)
        .with_context(|| format!("Directory {} not found", &args.backgrounds_path))?;

    let image_file_paths = get_valid_image_paths_from_provided_dir(&args.backgrounds_path)
        .with_context(|| format!("No images found in {}", &args.backgrounds_path))?;

    let mut event_listener = EventListener::new();
    event_listener.add_workspace_change_handler(move |data| {
        handle_workspace_change(data, &image_file_paths);
    });

    event_listener
        .start_listener()
        .context("Failed to start listener")?;

    Ok(())
}
