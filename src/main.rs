use crate::modules::app::App;
use crate::modules::cli::Cli;
use crate::modules::dir::Dir;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use log::{debug, error, info};
use std::env;
use std::error::Error;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

mod modules;

fn main() {
    let args = Cli::parse_args();
    let logger_level = if args.non_interactive() {
        args.verbose().log_level_filter()
    } else {
        log::LevelFilter::Off
    };
    env_logger::Builder::new().filter_level(logger_level).init();
    let current_path = env::current_dir().unwrap_or_else(|_| {
        error!("Failed to get current directory");
        std::process::exit(1);
    });
    let folder_name = args.name();
    info!(
        "Searching for \"{}\" in {}",
        folder_name,
        current_path.to_string_lossy()
    );
    let dirs = Dir::get_dirs(&current_path, None, folder_name.clone()).unwrap_or_else(|_| {
        error!("Failed to get directories");
        std::process::exit(1);
    });

    match args.non_interactive() {
        true => (),
        false => {
            show_ui(dirs).unwrap_or_else(|_| {
                error!("Failed to show UI");
                std::process::exit(1);
            });
            return;
        }
    }

    match args.dry_run() {
        true => dry_run(folder_name, &dirs),
        false => delete_directories(dirs),
    };
}

fn show_ui(dirs: Vec<Dir>) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(dirs);
    let res = app.run(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn dry_run(folder_name: &String, dirs: &Vec<Dir>) {
    let directory_text = if dirs.len() == 1 {
        "directory"
    } else {
        "directories"
    };
    println!(
        "Dry run:\n - {} \"{}\" {} found\n",
        dirs.len(),
        folder_name,
        directory_text
    );
    println!("Directories to delete:");
    for dir in dirs {
        println!(" - {dir}");
    }
}

fn delete_directories(dirs: Vec<Dir>) {
    let mut handles = Vec::new();
    debug!("Starting threads to delete directories");
    for mut dir in dirs {
        info!("Deleting {dir}");
        let handle = std::thread::spawn(move || {
            dir.delete_dir();
        });
        handles.push(handle);
    }
    debug!("All threads started");

    debug!("Waiting for threads to finish");
    for handle in handles.into_iter() {
        handle.join().unwrap_or_else(|_| {
            error!("Failed to join thread");
            std::process::exit(1);
        })
    }
    debug!("All threads finished");
}
