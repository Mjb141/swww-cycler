mod utils;

use clap::Parser;
use env_logger::Env;
use hyprland::event_listener::EventListener;
use hyprland::shared::WorkspaceType;
use log::{debug, error, warn};
use rand::{seq::SliceRandom, thread_rng};
use std::{path::PathBuf, process::Command};
use which::which;

use crate::utils::{get_valid_image_paths_from_provided_dir, selected_file_is_valid_img};

const SWWW_BINARY: &str = "swww";

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long)]
    pub backgrounds_path: String,
}

pub fn handle_workspace_change(data: WorkspaceType, valid_image_paths: &Vec<PathBuf>) {
    match data {
        WorkspaceType::Regular(reg_workspace_num) => {
            debug!(
                "Workspace change (Regular) to workspace {:?}",
                reg_workspace_num
            );

            let mut rng = thread_rng();
            let chosen_background = loop {
                match valid_image_paths.choose(&mut rng) {
                    Some(selected_file) => {
                        debug!("Selected (PathBuf): {:?}", selected_file);

                        if !selected_file_is_valid_img(selected_file) {
                            warn!(
                                "Selected file '{:?}' does not have an acceptable file extension.",
                                selected_file
                            );
                            continue;
                        }

                        let path = match selected_file.as_path().to_str() {
                            Some(path) => {
                                debug!("Converted (&str): {}", path);
                                path
                            }
                            None => {
                                warn!("Could not convert PathBuf to &str, selecting new file");
                                continue;
                            }
                        };

                        break path;
                    }
                    None => {
                        warn!("Couldn't select a file. Attempting to select another file");
                        continue;
                    }
                };
            };

            if let Err(_) = Command::new("swww")
                .args(["img", chosen_background])
                .output()
            {
                error!("Failed to issue 'swww' command. Not changing background.")
            }
        }
        WorkspaceType::Special(_) => {
            debug!("Workspace change event (Special) ignored");
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

    // Parse args and check provided dir exists
    let args = Args::parse();
    let backgrounds_dir = PathBuf::from(&args.backgrounds_path);
    if !backgrounds_dir.exists() | !backgrounds_dir.is_dir() {
        error!("Backgrounds directory {:?} not found", backgrounds_dir);
        return;
    };

    let valid_image_file_paths_in_provided_dir = match get_valid_image_paths_from_provided_dir(
        args.backgrounds_path,
    ) {
        Ok(vec_image_paths) => vec_image_paths,
        // Err(e) => panic!("Error: {e}"),
        Err(e) => match e {
            utils::ParsingError::PatternError(_) => {
                error!("Failed to extract Paths from provided directory. Please provide a valid directory path");
                return;
            }
            utils::ParsingError::NoValidFilesError(e) => {
                error!("{e}");
                return;
            }
            utils::ParsingError::NoValidImageFilesError(_) => {
                error!("{e}");
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
