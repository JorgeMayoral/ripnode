use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::{env, fs};

pub mod cli;

#[derive(Clone)]
pub struct NodeModulesDir {
    path: PathBuf,
    size: String,
}

impl NodeModulesDir {
    pub fn new(path: PathBuf) -> Self {
        let size = fs_extra::dir::get_size(&path).expect("Failed to get directory size");
        let size_str = bytesize::ByteSize::b(size).to_string();
        Self {
            path,
            size: size_str,
        }
    }

    pub fn get_node_modules_dirs(path: &Path, node_modules_dirs: Option<Vec<Self>>) -> Vec<Self> {
        let mut node_modules_dirs = node_modules_dirs.unwrap_or_default();
        let dirs = fs::read_dir(path).expect("Failed to read current directory");
        for dir in dirs {
            let dir = dir.expect("Failed to read directory");
            let dir_filename = dir.file_name();
            let dir_name = dir_filename.to_str().unwrap();
            if dir_name == "node_modules" {
                node_modules_dirs.push(Self::new(dir.path()));
            } else if dir.file_type().expect("Failed to get file type").is_dir() {
                node_modules_dirs =
                    Self::get_node_modules_dirs(&dir.path(), Some(node_modules_dirs.clone()));
            }
        }
        node_modules_dirs
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn size(&self) -> &String {
        &self.size
    }
}

impl Display for NodeModulesDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let relative_path = self
            .path
            .strip_prefix(env::current_dir().expect("Failed to get current directory"))
            .unwrap();
        let relative_path_string = format!("./{}", relative_path.display());
        write!(f, "{}: {}", relative_path_string, self.size)
    }
}
