use anyhow::{Context, Ok};
use clap::Parser;
use env_logger::Env;
use glob::glob;
use hyprland::event_listener::EventListener;
use hyprland::shared::WorkspaceType;
use log::{debug, error, info};
use rand::{seq::SliceRandom, thread_rng};
use std::{
    path::{Path, PathBuf},
    process::Command,
};
use which::which;

const SWWW_BINARY: &str = "swww";

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long)]
    pub backgrounds_path: String,
}

fn main() {
    let env = Env::default().filter_or("SWWW_CYCLER_LOG_LEVEL", "info");
    env_logger::init_from_env(env);

    if let Err(_) = which(SWWW_BINARY) {
        error!("'{SWWW_BINARY}' binary not found on PATH");
        return;
    }
    debug!("'{SWWW_BINARY}' binary found on PATH");

    let mut event_listener = EventListener::new();
    event_listener.add_workspace_change_handler(|data| handle_workspace_change(data));
    event_listener.start_listener().unwrap();
}

fn handle_workspace_change(data: WorkspaceType) {
    match data {
        WorkspaceType::Regular(reg_workspace_num) => {
            debug!(
                "Workspace change (Regular) to workspace {:?}",
                reg_workspace_num
            );
            let background_path = get_random_background_image().unwrap();
            debug!("Background selected: {:?}", background_path);
            change_background(background_path);
        }
        _ => {
            debug!("Workspace change event (Special) ignored");
        }
    }
}

fn get_random_background_image() -> anyhow::Result<PathBuf> {
    let args = Args::parse();
    let backgrounds_dir = Path::new(&args.backgrounds_path);
    if !backgrounds_dir.exists() | !backgrounds_dir.is_dir() {
        error!("Backgrounds directory {:?} not found", backgrounds_dir);
    };

    // See: https://docs.rs/glob/latest/glob/fn.glob.html#examples
    let backgrounds_paths: Vec<PathBuf> = glob(&format!("{}/*", args.backgrounds_path))
        .unwrap()
        .filter_map(Result::ok)
        .collect();

    let mut rng = thread_rng();
    let chosen_background = backgrounds_paths
        .choose(&mut rng)
        .context("Couldn't choose a background")?;
    Ok(chosen_background.clone())
}

fn change_background(background_path: PathBuf) {
    let path_str = background_path.as_path().to_str().unwrap();
    info!("Changing background to: {path_str}");
    Command::new("swww")
        .args(["img", path_str])
        .output()
        .unwrap();
}
