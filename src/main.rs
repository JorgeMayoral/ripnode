use ripnode::cli::Cli;
use ripnode::dir::Dir;
use std::{env, fs, thread};

fn main() {
    let args = Cli::parse_args();
    let current_path = env::current_dir().expect("Failed to get current directory");
    let folder_name = args.name();
    let dirs = Dir::get_dirs(&current_path, None, folder_name.clone());

    match args.dry_run() {
        true => dry_run(folder_name, &dirs),
        false => delete_directories(dirs),
    }
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
