// use walkdir::WalkDir;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize)]
pub struct FileNode {
    name: String,
    path: String,
    is_dir: bool,
    children: Option<Vec<FileNode>>,
}

pub fn build_tree(path: &PathBuf) -> FileNode {
    let name = path
					.file_name()
					.unwrap_or_default()
					.to_string_lossy()
					.into_owned();
    let path_str = path.strip_prefix("../root").unwrap_or(path).to_string_lossy().into_owned();
    let is_dir = path.is_dir();

    let children = if is_dir {
        let mut nodes = vec![];
        if let Ok(read_dir) = std::fs::read_dir(path) {
            for entry in read_dir.flatten() {
                let child_path = entry.path();
                nodes.push(build_tree(&child_path));
            }
        }
        Some(nodes)
    } else {
        None
    };

    FileNode { name, path: path_str, is_dir, children }
}
