use crate::dir::Dir;
use crate::ui::draw_ui;
use bytesize::ByteSize;
use crossterm::event::{Event, KeyCode};
use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::widgets::ListState;
use tui::Terminal;

pub struct StatefulList {
    pub state: ListState,
    pub items: Vec<Dir>,
}

impl StatefulList {
    pub fn with_items(items: Vec<Dir>) -> StatefulList {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
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

    pub fn previous(&mut self) {
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

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

pub struct App {
    pub title: &'static str,
    pub should_quit: bool,
    pub dirs: StatefulList,
    pub saved_space: u64,
}

impl App {
    pub fn new(dirs: Vec<Dir>) -> App {
        App {
            title: "

 ██▀███   ██▓ ██▓███   ███▄    █  ▒█████  ▓█████▄ ▓█████
▓██ ▒ ██▒▓██▒▓██░  ██▒ ██ ▀█   █ ▒██▒  ██▒▒██▀ ██▌▓█   ▀
▓██ ░▄█ ▒▒██▒▓██░ ██▓▒▓██  ▀█ ██▒▒██░  ██▒░██   █▌▒███
▒██▀▀█▄  ░██░▒██▄█▓▒ ▒▓██▒  ▐▌██▒▒██   ██░░▓█▄   ▌▒▓█  ▄
░██▓ ▒██▒░██░▒██▒ ░  ░▒██░   ▓██░░ ████▓▒░░▒████▓ ░▒████▒
░ ▒▓ ░▒▓░░▓  ▒▓▒░ ░  ░░ ▒░   ▒ ▒ ░ ▒░▒░▒░  ▒▒▓  ▒ ░░ ▒░ ░
  ░▒ ░ ▒░ ▒ ░░▒ ░     ░ ░░   ░ ▒░  ░ ▒ ▒░  ░ ▒  ▒  ░ ░  ░
  ░░   ░  ▒ ░░░          ░   ░ ░ ░ ░ ░ ▒   ░ ░  ░    ░
   ░      ░                    ░     ░ ░     ░       ░  ░
                                           ░

    ",
            should_quit: false,
            dirs: StatefulList::with_items(dirs),
            saved_space: 0,
        }
    }

    pub fn on_up(&mut self) {
        self.dirs.previous();
    }

    pub fn on_down(&mut self) {
        self.dirs.next();
    }

    pub fn on_quit(&mut self) {
        self.should_quit = true;
    }

    pub fn on_delete(&mut self) {
        let selected = self.dirs.state.selected();
        if let Some(selected) = selected {
            let dir = &self.dirs.items[selected];
            self.saved_space += dir.size().parse::<ByteSize>().unwrap().0;
            let _ = dir.delete_dir();
            self.dirs.items.remove(selected);
        }
    }

    pub fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    ) -> std::io::Result<()> {
        loop {
            if self.should_quit {
                break;
            }

            terminal.draw(|f| draw_ui(f, self))?;

            if let Event::Key(key) = crossterm::event::read()? {
                match key.code {
                    KeyCode::Char('q') => self.on_quit(),
                    KeyCode::Left => self.dirs.unselect(),
                    KeyCode::Down => self.on_down(),
                    KeyCode::Up => self.on_up(),
                    KeyCode::Enter | KeyCode::Backspace => self.on_delete(),
                    _ => {}
                }
            }
        }

        Ok(())
    }
}
