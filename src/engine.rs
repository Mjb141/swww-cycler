use anyhow::Context;
use std::process::Command;

use crate::Application;

pub fn get_engine(binary: Application) -> Box<dyn Engine> {
    match binary {
        Application::Swww => Box::new(Swww {
            which: String::from("swww"),
            prefix: String::from("swww"),
            args: vec![String::from("img")],
        }),
        Application::Hyprpaper => Box::new(Hyprpaper {
            which: String::from("hyprpaper"),
            prefix: String::from("hyprctl"),
            args: vec![String::from("hyprpaper"), String::from("wallpaper")],
        }),
    }
}

pub trait Engine {
    fn which(&self) -> &String;
    fn prefix(&self) -> &String;
    fn args(&self) -> &Vec<String>;
    fn change(&self, wallpaper: &str) -> anyhow::Result<()> {
        println!("wallpaper in send: {}", wallpaper);
        println!("self.prefix(): {}", self.prefix());
        println!("self.args(): {:?}", self.args());

        let mut command = Command::new(self.prefix());

        for arg in self.args() {
            println!("adding arg: {:?}", arg);
            command.arg(arg);
        }

        command
            .arg(wallpaper)
            .output()
            .context(format!("Failed to issue '{}' command", &self.which()))?;

        println!("command sent");

        Ok(())
    }
}

pub struct Swww {
    pub which: String,
    pub prefix: String,
    pub args: Vec<String>,
}
impl Engine for Swww {
    fn which(&self) -> &String {
        &self.which
    }
    fn prefix(&self) -> &String {
        &self.prefix
    }
    fn args(&self) -> &Vec<String> {
        &self.args
    }
}

pub struct Hyprpaper {
    pub which: String,
    pub prefix: String,
    pub args: Vec<String>,
}
impl Engine for Hyprpaper {
    fn which(&self) -> &String {
        &self.which
    }
    fn prefix(&self) -> &String {
        &self.prefix
    }
    fn args(&self) -> &Vec<String> {
        &self.args
    }
}
