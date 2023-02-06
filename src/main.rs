use std::process::Command;

fn main() {
    println!("This program will self-destruct 💀.");

    // get the path to the executable
    let executable_path: String = std::env::args().collect();

    // ...
    // main
    // program
    // ...

    // remove the executable
    if cfg!(target_os = "windows") {
        Command::new("del").arg(&executable_path).output().expect("Failed to remove executable.");
    } else if cfg!(target_os = "macos") {
        Command::new("rm").arg(&executable_path).output().expect("Failed to remove executable.");
    } else if cfg!(target_os = "linux") {
        Command::new("rm").arg(&executable_path).output().expect("Failed to remove executable.");
    } else {
        panic!("Unrecognised target OS.");
    }
}
