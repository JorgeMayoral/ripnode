use crate::modules::app::App;
use crate::modules::dir::Dir;
use bytesize::ByteSize;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Frame;

pub fn draw_ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(8),
                Constraint::Length(2),
                Constraint::Min(1),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    // Draw title
    let paragraph = Paragraph::new(app.title)
        .style(Style::default().fg(Color::Red))
        .alignment(Alignment::Left);
    f.render_widget(paragraph, chunks[0]);

    // Draw total size
    let total_size = Dir::sum_dirs_size(&app.dirs.items);
    let saved_space = ByteSize::b(app.saved_space).to_string();
    let text = vec![
        Line::from(format!("Total size: {}", total_size)),
        Line::from(format!("Saved space: {}", saved_space)),
    ];
    let paragraph = Paragraph::new(text)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left);
    f.render_widget(paragraph, chunks[1]);

    // Draw dirs list
    let items: Vec<ListItem> = app
        .dirs
        .items
        .iter()
        .map(|i| {
            let is_deleting = i.is_deleting();
            let is_deleted = i.is_deleted();
            let item_text = if is_deleting {
                format!("{i} [DELETING...]")
            } else if is_deleted {
                format!("{i} [DELETED]")
            } else {
                i.to_string()
            };
            let item_style = if is_deleting {
                Style::default().fg(Color::Yellow)
            } else if is_deleted {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(item_text).style(item_style)
        })
        .collect();

    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Directories"))
        .highlight_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
    f.render_stateful_widget(items, chunks[2], &mut app.dirs.state);

    // Draw help
    let paragraph = Paragraph::new("Select: <↑↓>\nDelete: <enter>\nQuit: <q>")
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left);
    f.render_widget(paragraph, chunks[3]);
}
