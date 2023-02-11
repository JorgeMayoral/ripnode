use std::{env, fs, thread};
use ripnode::NodeModulesDir;

fn main() {
    let current_path = env::current_dir().expect("Failed to get current directory");
    let node_modules_dirs = NodeModulesDir::get_node_modules_dirs(&current_path, None);

    let mut handles = Vec::new();
    for dir in node_modules_dirs {
        println!("Deleting {}: {} bytes", dir.path().display(), dir.size());
        let handle = thread::spawn(move || {
            fs::remove_dir_all(dir.path()).expect("Failed to delete directory");
        });
        handles.push(handle);
    }
    for handle in handles.into_iter() {
        handle.join().expect("Failed to join thread");
    }
}
