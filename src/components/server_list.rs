use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{action::Action, config::Config};

#[derive(Default)]
pub struct ServerList {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    state: ListState,
    items: Vec<String>,
}

impl ServerList {
    pub fn new() -> Self {
        Self {
            items: vec![
                "Server A".to_string(),
                "Server B".to_string(),
                "Server C".to_string(),
            ],
            ..Default::default()
        }
    }

    fn select_next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn select_previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

impl Component for ServerList {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> color_eyre::Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> color_eyre::Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        match action {
            Action::Up => self.select_previous(),
            Action::Down => self.select_next(),
            Action::Left => {
                // 左キーで「戻る（Homeへ）」アクションを発行する例
                // Actionに `GoToHome` などを定義しておくと良い
                return Ok(Some(Action::Help)); // 仮でHelpにしておく
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|i| ListItem::new(i.as_str()))
            .collect();
        let list = List::new(items)
            .block(Block::default().title("Server List").borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::White).fg(Color::Black));

        frame.render_stateful_widget(list, area, &mut self.state);
        Ok(())
    }
}
