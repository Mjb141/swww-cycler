use anyhow::Context;
use clap::Parser;
use env_logger::Env;
use glob::glob;
use hyprland::event_listener::EventListener;
use hyprland::shared::WorkspaceType;
use log::{debug, error};
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

pub fn handle_workspace_change(data: WorkspaceType, backgrounds_dir: &String) {
    match data {
        WorkspaceType::Regular(reg_workspace_num) => {
            debug!(
                "Workspace change (Regular) to workspace {:?}",
                reg_workspace_num
            );

            let backgrounds_paths: Vec<PathBuf> = glob(&format!("{}/*", backgrounds_dir))
                .unwrap()
                .filter_map(Result::ok)
                .collect();

            let mut rng = thread_rng();

            let chosen_background = backgrounds_paths
                .choose(&mut rng)
                .context("Couldn't choose a background")
                .unwrap()
                .as_path()
                .to_str()
                .unwrap();
            debug!("Background selected: {:?}", chosen_background);

            Command::new("swww")
                .args(["img", chosen_background])
                .output()
                .unwrap();
        }
        _ => {
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

    let args = Args::parse();
    let backgrounds_dir = Path::new(&args.backgrounds_path);
    if !backgrounds_dir.exists() | !backgrounds_dir.is_dir() {
        error!("Backgrounds directory {:?} not found", backgrounds_dir);
    };

    let mut event_listener = EventListener::new();
    event_listener.add_workspace_change_handler(move |data| {
        handle_workspace_change(data, &args.backgrounds_path)
    });
    event_listener.start_listener().unwrap();
}
