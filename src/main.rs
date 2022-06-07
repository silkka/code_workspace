use std::path::Path;
use std::{fs};

use std::fs::ReadDir;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Workspace {
    folder: String,
}

#[derive(Serialize, Deserialize)]
struct Alfred_Item {
    uid: String,
    title: String
}

fn main() {
    const WORSPACE_LOCATION: &str="/Users/anttipeltola/Library/Application Support/Code/User/workspaceStorage";
    // a8deed49d145245ed73e0540da56796e/workspace.json";

    let files: ReadDir= match fs::read_dir(WORSPACE_LOCATION) {
        Ok(files) => files,
        Err(_) => panic!(),
    };

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
        println!("{}", &space);

        if Path::new(&space).exists() {
            existing_worskpaces.push(space.to_string());
        }
    }
    

    println!("Existing ones:");

    for space in existing_worskpaces {
        println!("{}", space);
    }
}