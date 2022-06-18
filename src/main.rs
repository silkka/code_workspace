use std::path::Path;
use std::{fs};

use std::fs::ReadDir;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Workspace {
    folder: String,
}

#[derive(Serialize, Deserialize)]
struct AlfredItem {
    uid: String,
    title: String,
    arg: String
}

#[derive(Serialize, Deserialize)]
struct Items {
    items: Vec<AlfredItem>
}

fn get_workspace_folders(workspace_location: &str) -> ReadDir {
    if let Ok(files) = fs::read_dir(workspace_location) {
        files
    } else {
        panic!("Workspace folder is unreadable. Check path and permissions")
    }
}

fn main() {
    const WORSPACE_LOCATION: &str="/Users/anttipeltola/Library/Application Support/Code/User/workspaceStorage";
    // a8deed49d145245ed73e0540da56796e/workspace.json";

    let files = get_workspace_folders(WORSPACE_LOCATION);

    let mut worskpaces: Vec<String> = Vec::new();
    
    for file in files {
        let mut file = file.unwrap().path();
        file.push("workspace.json");

        // println!("{}", &file.display());

        let contents : String= match fs::read_to_string(file) {
            Ok(contents) => contents,
            Err(_) => continue,
        };
    
        // println!("{}", &contents);

        let w: Workspace= match serde_json::from_str(&contents) {
            Ok(w) => w,
            Err(_) => panic!(),
        };

        // println!("{}", &w.folder);

        worskpaces.push(w.folder);
    }

    let mut existing_worskpaces: Vec<String> = Vec::new();

    for space in worskpaces {
        let space = &space[7..];
        //println!("{}", &space);

        if Path::new(&space).exists() {
            existing_worskpaces.push(space.to_string());
        }
    }
    

    //println!("Existing ones:");

    let mut aflred_output: Vec<AlfredItem> = Vec::new();

    for space in existing_worskpaces {
        // println!("{}", space);
        let space = AlfredItem {
            title: String::from(&space),
            uid: String::from(&space),
            arg: String::from(&space),

        };
        aflred_output.push(space);
    }

    let alfred_output: Items = Items { items:aflred_output };

    let j = serde_json::to_string_pretty(&alfred_output).unwrap();

    println!("{}", j);

}
