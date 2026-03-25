use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{action::Action, config::Config};

#[derive(Default)]
pub struct Settings {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    state: ListState,
    items: Vec<String>,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            items: vec![
                "General".to_string(),
                "Path".to_string(),
                "Java".to_string(),
            ],
            ..Default::default()
        }
    }
    pub fn get_selected(&self) -> Option<&String> {
        self.state.selected().and_then(|i| self.items.get(i))
    }
    pub fn row_down(&mut self) {
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

    pub fn row_up(&mut self) {
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

impl Component for Settings {
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
            Action::Up => self.row_up(),
            Action::Down => self.row_down(),
            Action::Left => {
                return Ok(Some(Action::Help));
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
            .block(Block::default().title("Settings").borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::White).fg(Color::Black));

        frame.render_stateful_widget(list, area, &mut self.state);
        Ok(())
    }
}
