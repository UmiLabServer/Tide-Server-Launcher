use crate::app::{App, ServerStatus};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Cell, List, ListItem, Paragraph, Row, Table, Tabs},
};

pub fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title = Paragraph::new("ULS Server Launcher")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White)),
        );
    f.render_widget(title, chunks[0]);

    let menu_items: Vec<Line> = app.menu.iter().map(|t| Line::from(*t)).collect();
    let menu = Tabs::new(menu_items)
        .block(Block::default().borders(Borders::ALL).title("Menu"))
        .select(app.locate[app.depth])
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(chunks[1]);

    f.render_widget(menu, main_chunks[0]);

    match (app.depth, app.locate[app.depth]) {
        // menu wo 登録？
        (0, 0) => MainRender::servers(f, main_chunks[1], app.clone()),
        (0, 1) => MainRender::preferences(f, main_chunks[1], app),
        (1, 0) => EditRender::logs(f, main_chunks[1], app),
        (1, 1) => EditRender::mods(f, main_chunks[1], app),
        (1, 2) => EditRender::config(f, main_chunks[1], app),
        (1, 3) => EditRender::world(f, main_chunks[1], app),
        (1, 4) => EditRender::settings(f, main_chunks[1], app),
        _ => {}
    }

    let help_text = "Press ↑↓ to move list, Press ←→ to move menu, Enter to edit Server, q to quit";
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Help"));
    f.render_widget(help, chunks[2]);
}

struct MainRender;
struct EditRender;

impl MainRender {
    fn servers(f: &mut Frame, area: Rect, app: App) {
        let header = Row::new(vec!["Name", "Host", "Port", "Status"])
            .style(Style::default().fg(Color::Yellow))
            .height(1)
            .bottom_margin(1);
        let rows: Vec<Row> = app
            .items
            .iter()
            .enumerate()
            .map(|(i, server)| {
                let style = if i == app.item[app.depth] {
                    Style::default().bg(Color::DarkGray).fg(Color::White)
                } else {
                    Style::default()
                };

                Row::new(vec![
                    Cell::from(server.name.as_str()),
                    Cell::from(server.host.as_str()),
                    Cell::from(server.port.to_string()),
                ])
                .style(style)
            })
            .collect();

        let table = Table::new(rows)
            .widths(&[
                Constraint::Percentage(30),
                Constraint::Percentage(25),
                Constraint::Percentage(15),
                Constraint::Percentage(30),
            ])
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Server List")
                    .border_style(Style::default().fg(Color::White)),
            )
            .highlight_style(Style::default().bg(Color::DarkGray));
        f.render_widget(table, area);
    }
    fn preferences(f: &mut Frame, area: Rect, _app: &App) {
        let items: Vec<ListItem> = vec![
            ListItem::new(""),
            ListItem::new("Preference 2: ..."),
            ListItem::new("Preference 3: ..."),
        ];
        let preferences = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Preferences")
                .border_style(Style::default().fg(Color::White)),
        );
        f.render_widget(preferences, area);
    }
}

impl EditRender {
    fn logs(f: &mut Frame, area: Rect, app: &App) {
        let items: Vec<ListItem> = vec![
            ListItem::new(""),
            ListItem::new("Preference 2: ..."),
            ListItem::new("Preference 3: ..."),
        ];
        let logs = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Selected Server: {}", app.selected_server_name))
                .border_style(Style::default().fg(Color::White)),
        );
        f.render_widget(logs, area);
    }
    fn mods(f: &mut Frame, area: Rect, app: &App) {
        let items: Vec<ListItem> = vec![
            ListItem::new(""),
            ListItem::new("Preference 2: ..."),
            ListItem::new("Preference 3: ..."),
        ];
        let mods = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Selected Server: {}", app.selected_server_name))
                .border_style(Style::default().fg(Color::White)),
        );
        f.render_widget(mods, area);
    }
    fn config(f: &mut Frame, area: Rect, app: &App) {
        let items: Vec<ListItem> = vec![
            ListItem::new(""),
            ListItem::new("Preference 2: ..."),
            ListItem::new("Preference 3: ..."),
        ];
        let config = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Selected Server: {}", app.selected_server_name))
                .border_style(Style::default().fg(Color::White)),
        );
        f.render_widget(config, area);
    }
    fn world(f: &mut Frame, area: Rect, app: &App) {
        let items: Vec<ListItem> = vec![
            ListItem::new(""),
            ListItem::new("Preference 2: ..."),
            ListItem::new("Preference 3: ..."),
        ];
        let world = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Selected Server: {}", app.selected_server_name))
                .border_style(Style::default().fg(Color::White)),
        );
        f.render_widget(world, area);
    }
    fn settings(f: &mut Frame, area: Rect, app: &App) {
        let items: Vec<ListItem> = vec![
            ListItem::new(""),
            ListItem::new("Preference 2: ..."),
            ListItem::new("Preference 3: ..."),
        ];
        let settings = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Selected Server: {}", app.selected_server_name))
                .border_style(Style::default().fg(Color::White)),
        );
        f.render_widget(settings, area);
    }
}