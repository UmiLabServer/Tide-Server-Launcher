use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{action::Action, config::Config};

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for Home {
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
            Action::Tick => {
                // add any logic here that should run on every tick
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(6), Constraint::Length(3),  Constraint::Min(10)])
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
        frame.render_widget(
            Paragraph::new("Menu").block(Block::new().borders(Borders::ALL)),
            menu[0],
        );
        frame.render_widget(
            Paragraph::new("Main").block(Block::new().borders(Borders::ALL)),
            main[0],
        );
        Ok(())
    }
}
