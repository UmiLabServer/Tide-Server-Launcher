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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppMode {
    List,
    Detail,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MainTab {
    Servers,
    Preference,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DetailTab {
    Logs,
    Mods,
    Config,
    World,
    Settings,
}

#[derive(Clone)]
pub struct App {
    pub items: Vec<ServerConfig>,
    pub mode: AppMode,
    pub main_tab: MainTab,
    pub detail_tab: DetailTab,
    pub selected_server_index: usize,
    pub selected_server_name: String,
}

impl App {
    pub fn new() -> Self {
        let items = Self::load_config().unwrap_or_else(|_| vec![]);
        Self {
            items,
            mode: AppMode::List,
            main_tab: MainTab::Servers,
            detail_tab: DetailTab::Logs,
            selected_server_index: 0,
            selected_server_name: String::new(),
        }
    }

    pub fn menu_items(&self) -> Vec<&'static str> {
        match self.mode {
            AppMode::List => vec!["Servers", "Preference"],
            AppMode::Detail => vec!["Logs", "Mods", "Config", "World", "Settings"],
        }
    }

    pub fn current_menu_index(&self) -> usize {
        match self.mode {
            AppMode::List => match self.main_tab {
                MainTab::Servers => 0,
                MainTab::Preference => 1,
            },
            AppMode::Detail => match self.detail_tab {
                DetailTab::Logs => 0,
                DetailTab::Mods => 1,
                DetailTab::Config => 2,
                DetailTab::World => 3,
                DetailTab::Settings => 4,
            },
        }
    }

    pub fn set_menu_index(&mut self, index: usize) {
        if self.mode == AppMode::List {
            self.main_tab = match index {
                0 => MainTab::Servers,
                1 => MainTab::Preference,
                _ => MainTab::Servers,
            };
        } else {
            self.detail_tab = match index {
                0 => DetailTab::Logs,
                1 => DetailTab::Mods,
                2 => DetailTab::Config,
                3 => DetailTab::World,
                4 => DetailTab::Settings,
                _ => DetailTab::Logs,
            };
        }
    }

    pub fn next_menu(&mut self) {
        let max = self.menu_items().len();
        let current = self.current_menu_index();
        let next = (current + 1) % max;
        self.set_menu_index(next);
    }

    pub fn previous_menu(&mut self) {
        let max = self.menu_items().len();
        let current = self.current_menu_index();
        let previous = if current == 0 { max - 1 } else { current - 1 };
        self.set_menu_index(previous);
    }

    pub fn next_server(&mut self) {
        if self.items.is_empty() {
            return;
        }
        self.selected_server_index = (self.selected_server_index + 1) % self.items.len();
    }

    pub fn previous_server(&mut self) {
        if self.items.is_empty() {
            return;
        }
        if self.selected_server_index == 0 {
            self.selected_server_index = self.items.len() - 1;
        } else {
            self.selected_server_index -= 1;
        }
    }

    pub fn enter_detail(&mut self) {
        if self.mode == AppMode::List && self.main_tab == MainTab::Servers && !self.items.is_empty() {
            self.selected_server_name = self.items[self.selected_server_index].name.clone();
            let _ = self.save_config();
            self.mode = AppMode::Detail;
            self.detail_tab = DetailTab::Logs;
        }
    }

    pub fn back_to_list(&mut self) {
        self.mode = AppMode::List;
        self.main_tab = MainTab::Servers;
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
