use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::fs::ReadDir;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Workspace {
    folder: String,
}

impl Workspace {
    /// Returns a reference to the folder path of this [`Workspace`].
    /// Removes file:// from the start of the path
    fn folder_path(&self) -> &str {
        const FILE_PREFIX: &str = "file://";
        if self.folder.starts_with(FILE_PREFIX) {
            return &self.folder[FILE_PREFIX.len()..];
        }
        &self.folder
    }
}

#[derive(Serialize, Deserialize)]
struct AlfredItem {
    uid: String,
    title: String,
    subtitle: String,
    arg: String,
}

impl AlfredItem {
    fn new(path: String) -> Self {
        let title = path.split('/').last().unwrap_or(&path);
        Self {
            uid: String::from(&path),
            title: String::from(title),
            subtitle: String::from(&path),
            arg: String::from(&path),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Items {
    items: Vec<AlfredItem>,
}

#[derive(Debug, Parser)]
#[clap(
    author = "Silkka",
    version = "1",
    about = "List VSCode workspaces for Alfred's Script Filter"
)]
struct Args {
    #[clap(
        short,
        long,
        value_parser,
        help = "Full path to workspace storage (Code/User/workspaceStorage)"
    )]
    workspace_storage: String,
}

fn main() {
    let args = Args::parse();

    // Parse the worskpace locations from worspaceStorage
    let folders = get_workspace_folders(args.workspace_storage);
    let mut worskpaces: Vec<String> = get_workspace_locations(folders);
    worskpaces.retain(|w| Path::new(&w).exists()); // Drop workspaces that don't exist

    // Construct Alfred output json
    let aflred_output: Vec<AlfredItem> =
        worskpaces.into_iter().map(|x| AlfredItem::new(x)).collect();
    let alfred_output: Items = Items {
        items: aflred_output,
    };
    let alfred_output = json!(alfred_output);

    println!("{:#}", alfred_output);
}

/// Returns iterator of the contents of the workspace storage
///
/// # Panics
///
/// Panics if the workspace storage folder can't be read.
fn get_workspace_folders(workspace_location: String) -> ReadDir {
    if let Ok(files) = fs::read_dir(workspace_location) {
        files
    } else {
        panic!("Workspace folder is unreadable. Check path and permissions")
    }
}

/// Returns a vector with vscode workspace paths.
/// Format: /vscode/workspace/locaton
/// # Arguments
/// * `folders` ReadDir iterator for Code/User/workspaceStorage
fn get_workspace_locations(folders: ReadDir) -> Vec<String> {
    let mut worskpaces: Vec<String> = Vec::new();

    for folder in folders {
        // Get the workspace.json file contents
        let contents = match get_workspace_json_contents(folder) {
            Some(value) => value,
            None => continue,
        };
        // Parse workspace location from workspace.json
        let w: Workspace = match serde_json::from_str(&contents) {
            Ok(w) => w,
            Err(_) => continue,
        };
        worskpaces.push(w.folder_path().to_string());
    }

    worskpaces
}

/// Returns the contents of a workspace.json file
/// # Arguments
/// * folder: vscode worskpace folder with worskpace.json file inside
fn get_workspace_json_contents(folder: Result<fs::DirEntry, std::io::Error>) -> Option<String> {
    let mut file = match folder {
        Ok(t) => t,
        Err(_) => return None,
    }
    .path();
    file.push("workspace.json");
    let contents: String = match fs::read_to_string(file) {
        Ok(contents) => contents,
        Err(_) => return None,
    };
    Some(contents)
}
