use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub dir_name: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ServerStatus {
    Running,
    Stopped,
    Starting,
    Error,
}

impl ServerStatus {
    /*pub fn as_str(&self) -> &'static str {
        match self {
            ServerStatus::Running => "Running",
            ServerStatus::Stopped => "Stopped",
            ServerStatus::Starting => "Starting...",
            ServerStatus::Error => "Error",
        }
    }

    pub fn as_str_animated(&self, tick_count: usize) -> String {
        match self {
            ServerStatus::Running => "Running".to_string(),
            ServerStatus::Stopped => "Stopped".to_string(),
            ServerStatus::Starting => {
                let dots = match tick_count % 4 {
                    0 => "",
                    1 => ".",
                    2 => "..",
                    3 => "...",
                    _ => "",
                };
                format!("Starting{}", dots)
            }
            ServerStatus::Error => "Error".to_string(),
        }
    }*/
}

#[derive(Clone)]
pub struct App {
    pub items: Vec<ServerConfig>,
    //pub tick_count: usize,
    pub locate: [usize; 2],
    pub item: [usize; 2],
    pub depth: usize,
    pub menu: Vec<&'static str>,
    pub selected_server_name: String,
}

impl App {
    pub fn new() -> Self {
        let items = Self::load_config().unwrap_or_else(|_| vec![]);
        Self {
            items,
            //tick_count: 0,
            locate: [0, 0],
            item: [0, 0],
            depth: 0,
            menu: vec![],
            selected_server_name: String::new(),
        }
    }

    const CONFIG_FILE: &'static str = "servers.json";

    pub fn save_config(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(&self.items)?;
        fs::write(Self::CONFIG_FILE, json)?;
        Ok(())
    }

    pub fn load_config() -> Result<Vec<ServerConfig>, Box<dyn std::error::Error>> {
        if !Path::new(Self::CONFIG_FILE).exists() {
            return Err("Config file does not exist".into());
        }
        let content = fs::read_to_string(Self::CONFIG_FILE)?;
        let items: Vec<ServerConfig> = serde_json::from_str(&content)?;
        Ok(items)
    }
}
