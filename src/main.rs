use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Workspace {
    folder: String,
}

fn main() {
    const WORSPACE_LOCATION: &str="/Users/anttipeltola/Library/Application Support/Code/User/workspaceStorage/a8deed49d145245ed73e0540da56796e/workspace.json";
    let contents = fs::read_to_string(WORSPACE_LOCATION)
        .expect("Failed to read file");
    
    println!("{}", &contents);

    let w:Workspace= match serde_json::from_str(&contents) {
        Ok(w) => w,
        Err(_) => panic!(),
    };

    println!("{}", &w.folder);
}
