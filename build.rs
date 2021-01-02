use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let client_dir = env::var("CLIENT_DIR").unwrap_or("client".to_string());
    let root = Path::new(&client_dir);
    assert!(env::set_current_dir(&root).is_ok());
    Command::new("yarn").status().unwrap();
    Command::new("yarn").args(&["build"]).status().unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=client/package.json");
}
