use ripnode::cli::Cli;
use ripnode::dir::Dir;
use std::{env, fs, thread};

fn main() {
    let args = Cli::parse_args();
    let current_path = env::current_dir().expect("Failed to get current directory");
    let folder_name = args.name();
    let dirs = Dir::get_dirs(&current_path, None, folder_name.clone());

    match args.dry_run() {
        true => {
            println!(
                "Dry run: {} \"{}\" directories found",
                dirs.len(),
                folder_name
            );
            for dir in dirs {
                println!("{dir}");
            }
        }
        false => {
            let mut handles = Vec::new();
            for dir in dirs {
                println!("Deleting {dir}");
                let handle = thread::spawn(move || {
                    fs::remove_dir_all(dir.path()).expect("Failed to delete directory");
                });
                handles.push(handle);
            }
            for handle in handles.into_iter() {
                handle.join().expect("Failed to join thread");
            }
        }
    }
}
