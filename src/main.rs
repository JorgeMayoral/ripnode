use std::path::{Path, PathBuf};
use std::{env, fs, thread};

struct NodeModulesDir {
    path: PathBuf,
    size: String,
}

impl NodeModulesDir {
    fn new(path: PathBuf) -> Self {
        let size = fs_extra::dir::get_size(&path).expect("Failed to get directory size");
        let size_str = bytesize::ByteSize::b(size).to_string();
        Self { path, size: size_str }
    }
}

fn get_node_modules_dirs(path: &Path, node_modules_dirs: &mut Vec<NodeModulesDir>) {
    let dirs = fs::read_dir(path).expect("Failed to read current directory");
    for dir in dirs {
        let dir = dir.expect("Failed to read directory");
        let dir_filename = dir.file_name();
        let dir_name = dir_filename.to_str().unwrap();
        if dir_name == "node_modules" {
            node_modules_dirs.push(NodeModulesDir::new(dir.path()));
        } else if dir.file_type().expect("Failed to get file type").is_dir() {
            get_node_modules_dirs(&dir.path(), node_modules_dirs);
        }
    }
}

fn main() {
    let current_path = env::current_dir().expect("Failed to get current directory");
    let mut node_modules_dirs = Vec::new();
    get_node_modules_dirs(&current_path, &mut node_modules_dirs);

    let mut handles = Vec::new();
    for dir in node_modules_dirs {
        println!("Deleting {}: {} bytes", dir.path.display(), dir.size);
        let handle = thread::spawn(move || {
            fs::remove_dir_all(dir.path).expect("Failed to delete directory");
        });
        handles.push(handle);
    }
    for handle in handles.into_iter() {
        handle.join().expect("Failed to join thread");
    }
}
