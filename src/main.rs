use anyhow::Context;
use clap::Parser;
use env_logger::Env;
use glob::glob;
use hyprland::event_listener::EventListener;
use hyprland::shared::WorkspaceType;
use log::{debug, error};
use rand::{seq::SliceRandom, thread_rng};
use std::{path::PathBuf, process::Command};
use which::which;

const SWWW_BINARY: &str = "swww";
const ACCEPTED_FILE_EXTS: [&'static str; 3] = ["webp", "jpg", "jpeg"];

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long)]
    pub backgrounds_path: String,
}

pub fn handle_workspace_change(
    data: WorkspaceType,
    backgrounds_dir: &String,
) -> anyhow::Result<()> {
    match data {
        WorkspaceType::Regular(reg_workspace_num) => {
            debug!(
                "Workspace change (Regular) to workspace {:?}",
                reg_workspace_num
            );

            let files_objects_in_provided_dir: glob::Paths =
                glob(&format!("{}/*", backgrounds_dir))
                    .context("PatternError, please provide a valid --backgrounds-path value")?;

            let valid_file_objects_in_provided_dir: Vec<PathBuf> = files_objects_in_provided_dir
                .filter_map(Result::ok)
                .collect();

            let mut rng = thread_rng();

            let chosen_background = loop {
                match valid_file_objects_in_provided_dir.choose(&mut rng) {
                    Some(selected_file) => {
                        debug!("Selected (PathBuf): {:?}", selected_file);

                        let ext = match selected_file.extension() {
                            Some(ext) => {
                                match ext.to_str() {
                                    Some(str_ext) => str_ext,
                                    None => {
                                        debug!("Couldn't convert extension to &str. Selecting a new file.");
                                        continue;
                                    }
                                }
                            }
                            None => {
                                debug!("No file extension detected. Selecting a new file.");
                                continue;
                            }
                        };

                        if !ACCEPTED_FILE_EXTS.contains(&ext) {
                            debug!(
                                "'{}' is not an accepted file type. Selecting a new file.",
                                ext
                            );
                            continue;
                        }

                        let path = match selected_file.as_path().to_str() {
                            Some(path) => {
                                debug!("Converted (&str): {}", path);
                                path
                            }
                            None => {
                                error!("Could not convert, selecting new file");
                                continue;
                            }
                        };

                        break path;
                    }
                    None => {
                        error!("Couldn't select a file. Attempting to select another file");
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

            Ok(())
        }
        WorkspaceType::Special(_) => {
            debug!("Workspace change event (Special) ignored");
            Ok(())
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
    };

    let mut event_listener = EventListener::new();
    event_listener.add_workspace_change_handler(move |data| {
        if let Err(_) = handle_workspace_change(data, &args.backgrounds_path) {
            error!("Failed to handle workspace change event.")
        }
    });

    match event_listener.start_listener() {
        Ok(_) => debug!("Listener started"),
        Err(_) => {
            panic!("Failed to start listener")
        }
    };
}
