use crate::modules::app::App;
use crate::modules::dir::Dir;
use bytesize::ByteSize;
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::Spans;
use tui::widgets::{Block, Borders, List, ListItem, Paragraph};
use tui::Frame;

pub fn draw_ui(f: &mut Frame<CrosstermBackend<Stdout>>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(10),
                Constraint::Percentage(55),
                Constraint::Percentage(5),
            ]
            .as_ref(),
        )
        .split(f.size());

    // Draw title
    let paragraph = Paragraph::new(app.title)
        .style(Style::default().fg(Color::Red))
        .alignment(Alignment::Center);
    f.render_widget(paragraph, chunks[0]);

    // Draw total size
    let total_size = Dir::sum_dirs_size(&app.dirs.items);
    let saved_space = ByteSize::b(app.saved_space).to_string();
    let text = vec![
        Spans::from(format!("Total size: {}", total_size)),
        Spans::from(format!("Saved space: {}", saved_space)),
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
            ListItem::new(i.to_string()).style(Style::default())
        })
        .collect();

    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Directories"))
        .highlight_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");
    f.render_stateful_widget(items, chunks[2], &mut app.dirs.state);

    // Draw help
    let paragraph = Paragraph::new("Use arrow keys to navigate. Press 'enter' to delete selected directory. Press 'q' to quit.")
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left);
    f.render_widget(paragraph, chunks[3]);
}
