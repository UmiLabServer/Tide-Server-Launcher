use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::{server_list::ServerList, settings::Settings, Component};
use crate::{action::Action, config::Config};

#[derive(Debug, Default, Clone, PartialEq)]
enum HomeState {
    #[default]
    ServerMenu, // Servers, Settings
    DetailMenu, // Logs, Mods, Config, World, Preferences
}

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    server_list: ServerList,
    settings: Settings,
    state: HomeState,
    server_menu_state: usize,
    detail_menu_state: usize,
}

impl Home {
    pub fn new() -> Self {
        Self {
            server_list: ServerList::new(),
            settings: Settings::new(),
            detail_menu_state: 0,
            ..Default::default()
        }
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> color_eyre::Result<()> {
        self.command_tx = Some(tx.clone());
        self.server_list.register_action_handler(tx.clone())?;
        self.settings.register_action_handler(tx.clone())?;
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> color_eyre::Result<()> {
        self.config = config.clone();
        self.server_list.register_config_handler(config.clone())?;
        self.settings.register_config_handler(config)?;
        Ok(())
    }

    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        match action {
            Action::Tick => {
                // add any logic here that should run on every tick
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
            _ => {}
        }
        match self.state {
            HomeState::ServerMenu => match action {
                Action::Up | Action::Down => match self.server_menu_state {
                    0 => {
                        if let Some(act) = self.server_list.update(action.clone())? {
                            return Ok(Some(act));
                        }
                    }
                    1 => {
                        if let Some(act) = self.settings.update(action.clone())? {
                            return Ok(Some(act));
                        }
                    }
                    _ => {}
                },
                Action::Left => {
                    self.server_menu_state = (self.server_menu_state + 1) % 2;
                }
                Action::Right => {
                    self.server_menu_state = (self.server_menu_state + 1) % 2;
                }
                Action::Enter => {
                    if self.server_list.get_selected().is_some() {
                        self.state = HomeState::DetailMenu;
                    }
                }
                _ => {}
            },
            HomeState::DetailMenu => match action {
                Action::Esc => self.state = HomeState::ServerMenu,
                Action::Right => {
                    self.detail_menu_state = (self.detail_menu_state + 1) % 5;
                }
                Action::Left => {
                    self.detail_menu_state = (self.detail_menu_state + 4) % 5;
                }
                _ => {}
            },
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(6), // header
                Constraint::Length(3), // menu
                Constraint::Min(10),   // main
            ])
            .split(area);
        let header = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(40), Constraint::Percentage(50)])
            .split(vertical[0]);
        let menu = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(vertical[1]);
        let main = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(vertical[2]);
        frame.render_widget(
            Paragraph::new("Header").block(Block::new().borders(Borders::ALL)),
            header[0],
        );
        match self.state {
            HomeState::ServerMenu => {
                let detail_menu_items = vec!["Servers", "Settings"];
                let tabs = Tabs::new(detail_menu_items)
                    .block(Block::default().borders(Borders::ALL))
                    .select(self.server_menu_state)
                    .highlight_style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    )
                    .divider(Span::raw("|"));
                frame.render_widget(tabs, menu[0]);
                match self.server_menu_state {
                    0 => {
                        self.server_list.draw(frame, main[0])?;
                    }
                    1 => {
                        self.settings.draw(frame, main[0])?;
                    }
                    _ => {}
                }
            }
            HomeState::DetailMenu => {
                let detail_menu_items = vec!["Logs", "Mods", "Config", "World", "Preferences"];
                let tabs = Tabs::new(detail_menu_items)
                    .block(Block::default().borders(Borders::ALL))
                    .select(self.detail_menu_state)
                    .highlight_style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    )
                    .divider(Span::raw("|"));
                frame.render_widget(tabs, menu[0]);
            }
        }
        Ok(())
    }
}
