use bytesize::ByteSize;
use log::{error, info};
use std::error::Error;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::{env, fs};

#[derive(Clone)]
pub struct Dir {
    path: PathBuf,
    size: String,
}

impl Dir {
    pub fn new(path: PathBuf) -> Result<Self, Box<dyn Error>> {
        info!("Getting size of {}", path.to_string_lossy());
        let size = fs_extra::dir::get_size(&path)?;
        let size_str = bytesize::ByteSize::b(size).to_string();
        Ok(Self {
            path,
            size: size_str,
        })
    }

    pub fn get_dirs(
        path: &Path,
        found_dirs: Option<Vec<Self>>,
        target_dir_name: String,
    ) -> Result<Vec<Self>, Box<dyn Error>> {
        let mut found_dirs = found_dirs.unwrap_or_default();
        let dirs = fs::read_dir(path)?;
        for dir in dirs {
            let dir = dir?;
            let dir_filename = dir.file_name();
            let dir_name = dir_filename.to_str().unwrap();
            if dir_name == target_dir_name {
                found_dirs.push(Self::new(dir.path())?);
            } else if dir.file_type()?.is_dir() {
                found_dirs = Self::get_dirs(
                    &dir.path(),
                    Some(found_dirs.clone()),
                    target_dir_name.clone(),
                )?;
            }
        }
        Ok(found_dirs)
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn size(&self) -> &String {
        &self.size
    }

    pub fn sum_dirs_size(dirs: &Vec<Self>) -> String {
        let sum = dirs
            .iter()
            .fold(0, |acc, dir| dir.size.parse::<ByteSize>().unwrap().0 + acc);
        ByteSize::b(sum).to_string()
    }
}

impl Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let current_dir = env::current_dir().unwrap_or_else(|_| {
            error!("Failed to get current directory");
            std::process::exit(1);
        });
        let relative_path = self.path.strip_prefix(current_dir).unwrap();
        let relative_path_string = format!("./{}", relative_path.display());
        write!(f, "{}: {}", relative_path_string, self.size)
    }
}
