use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::{server_list::ServerList, Component};
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
    state: HomeState,
    detail_menu_state: usize,
}

impl Home {
    pub fn new() -> Self {
        Self {
            server_list: ServerList::new(),
            detail_menu_state: 0,
            ..Default::default()
        }
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> color_eyre::Result<()> {
        self.command_tx = Some(tx.clone());
        self.server_list.register_action_handler(tx.clone())?;
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> color_eyre::Result<()> {
        self.config = config.clone();
        self.server_list.register_config_handler(config)?;
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
                Action::Up => self.server_list.row_up(),
                Action::Down => self.server_list.row_down(),
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
                frame.render_widget(
                    Paragraph::new(format!(
                        "Selected: {}",
                        self.server_list.get_selected().unwrap_or(&"None".into())
                    ))
                    .block(Block::new().borders(Borders::ALL)),
                    menu[0],
                );
            }
            HomeState::DetailMenu => {
                let detail_menu_items = vec!["Logs", "Mods", "Config", "World", "Preferences"];
                let tabs = Tabs::new(detail_menu_items)
                    .block(Block::default().borders(Borders::ALL).title("Menu"))
                    .select(self.detail_menu_state) // これでハイライト位置を指定
                    .highlight_style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    )
                    .divider(Span::raw("|")); // 区切り文字

                frame.render_widget(tabs, menu[0]); // 1つのボックス(menu[0])に描画
            }
        }
        self.server_list.draw(frame, main[0])?;
        Ok(())
    }
}
