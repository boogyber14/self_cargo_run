use std::process::Command;
use std::env;

pub fn main() {
    // Check if the program is already running within a Cargo environment
    if env::var("CARGO").is_ok() {
        eprintln!("Program is already running within a Cargo environment. Exiting.");
        return;
    }

    // Get the current directory
    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    // Create a let binding to hold the value returned by current_exe()
    let exe_path = std::env::current_exe().expect("Failed to get current executable's path");

    // Extract the file name from the path
    let exe_name = exe_path
        .file_name()
        .expect("Failed to get executable's file name")
        .to_str()
        .expect("Failed to convert to string");

    // Build the command to run 'cargo run'
    let mut cmd = Command::new("cargo");
    cmd.arg("run").arg("--bin").arg(exe_name);

    // Execute the command in the current directory
    let status = cmd.current_dir(current_dir)
        .status()
        .expect("Failed to execute command");

    if !status.success() {
        eprintln!("Failed to run the program");
    }
}
