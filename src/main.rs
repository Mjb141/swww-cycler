mod utils;

use clap::Parser;
use env_logger::Env;
use hyprland::event_listener::EventListener;
use hyprland::shared::WorkspaceType;
use log::{debug, error};
use rand::{seq::SliceRandom, thread_rng};
use std::{path::PathBuf, process::Command};
use which::which;

use crate::utils::get_valid_image_paths_from_provided_dir;
use crate::utils::ParsingError;

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

fn main() {
    let env = Env::default().filter_or("SWWW_CYCLER_LOG_LEVEL", "info");
    env_logger::init_from_env(env);

    if let Err(_) = which(SWWW_BINARY) {
        error!("'{SWWW_BINARY}' binary not found on PATH");
        return;
    }
    debug!("'{SWWW_BINARY}' binary found on PATH");

    let args = Args::parse();
    let backgrounds_dir = PathBuf::from(&args.backgrounds_path);
    if !backgrounds_dir.exists() | !backgrounds_dir.is_dir() {
        error!("Backgrounds directory {:?} not found", backgrounds_dir);
        return;
    };

    let valid_image_file_paths_in_provided_dir = match get_valid_image_paths_from_provided_dir(
        args.backgrounds_path,
    ) {
        Ok(vec_image_paths) => {
            debug!(
                "Number of valid images in provided directory: {}",
                vec_image_paths.len()
            );
            vec_image_paths
        }
        Err(e) => match e {
            ParsingError::PatternError(_) => {
                error!("Failed to extract Paths from provided directory. Please provide a valid directory path");
                return;
            }
            ParsingError::NoValidImageFilesError => {
                error!("Didn't find any valid image files in provided directory. Please provide a valid directory path");
                return;
            }
        },
    };

    let mut event_listener = EventListener::new();
    event_listener.add_workspace_change_handler(move |data| {
        handle_workspace_change(data, &valid_image_file_paths_in_provided_dir);
    });

    match event_listener.start_listener() {
        Ok(_) => debug!("Listener started"),
        Err(_) => {
            panic!("Failed to start listener")
        }
    };
}
