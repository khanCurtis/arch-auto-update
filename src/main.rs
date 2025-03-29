use std::process::Command;

fn main() {
    println!("Starting System Update...");

    //update pacman packages
    println!("Updating Pacman Packages...");
    let update_result = Command::new("sudo")
        .args(&["pacman", "-Syu", "--noconfirm"])
        .output();

    match update_result {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            if output.status.success() {
                println!("Pacman Updates Complete!");
                println!("Output: {}", stdout);
            } else {
                println!("Pacman Updates Failed.");
                println!("Output: {}", stderr);
            }
        },
        Err(e) => {
            println!("Failed to execute update command: {}", e);
        }
    }

    //update AUR packages
    println!("Updating AUR Packages...");
    let aur_result = Command::new("yay") //change to your AUR helper
        .args(&["-Syu", "--noconfirm"])
        .output();

    match aur_result {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            if output.status.success() {
                println!("AUR Updates Complete!");
                println!("Output: {}", stdout);
            } else {
                println!("AUR Updates Failed.");
                println!("Output: {}", stderr);
            }
        },
        Err(e) => {
            println!("Failed to execute update command: {}", e);
        }
    }

    println!("System Update Complete!");
}