use std::process::Command;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    // Create a temporary script file
    let script_path = "/tmp/arch_update_script.sh";
    let mut file = File::create(script_path).expect("Failed to create script file");
    
    // Write commands to the script file
    writeln!(file, "#!/bin/bash").unwrap();
    writeln!(file, "echo 'Starting Arch Linux system update...'").unwrap();
    writeln!(file, "echo 'Running pacman -Syu...'").unwrap();
    writeln!(file, "sudo pacman -Syu --noconfirm").unwrap();
    writeln!(file, "echo 'Updating AUR packages...'").unwrap();
    writeln!(file, "yay -Syu --noconfirm --noclipboard --nodiffmenu --noeditmenu").unwrap();
    writeln!(file, "echo 'Update process completed'").unwrap();
    writeln!(file, "echo 'Terminal will close in 5 seconds...'").unwrap();
    writeln!(file, "sleep 5").unwrap();
    
    // Make the script executable
    Command::new("chmod")
        .args(&["+x", script_path])
        .status()
        .expect("Failed to make script executable");
    
    // Find which terminal emulator is available
    let terminal = find_terminal_emulator();
    
    // Launch terminal with the script
    match terminal {
        Some(term) => {
            println!("Launching terminal: {}", term);
            let status = if term == "gnome-terminal" || term == "konsole" {
                Command::new(term)
                    .args(&["--", "/bin/bash", script_path])
                    .status()
            } else if term == "xterm" || term == "urxvt" {
                Command::new(term)
                    .args(&["-e", "/bin/bash", script_path])
                    .status()
            } else if term == "alacritty" {
                Command::new(term)
                    .args(&["-e", "/bin/bash", script_path])
                    .status()
            } else {
                Command::new(term)
                    .args(&["-e", "/bin/bash", script_path])
                    .status()
            };
            
            match status {
                Ok(_) => println!("Terminal launched successfully"),
                Err(e) => println!("Failed to launch terminal: {}", e),
            }
        },
        None => {
            println!("No terminal emulator found. Running updates directly...");
            // Run updates directly
            run_updates();
        }
    }
}

fn find_terminal_emulator() -> Option<String> {
    let terminals = vec![
        "alacritty", "konsole", "gnome-terminal", "xfce4-terminal", 
        "mate-terminal", "terminator", "kitty", "urxvt", "xterm"
    ];
    
    for term in terminals {
        if Command::new("which")
            .arg(term)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false) {
            return Some(term.to_string());
        }
    }
    
    None
}

fn run_updates() {
    println!("Starting Arch Linux system update...");
    
    // Run pacman update command
    println!("Running pacman -Syu...");
    let update_result = Command::new("sudo")
        .args(&["pacman", "-Syu", "--noconfirm"])
        .status();
    
    match update_result {
        Ok(status) => {
            if status.success() {
                println!("System update completed successfully");
            } else {
                println!("System update failed");
            }
        },
        Err(e) => {
            println!("Failed to execute update command: {}", e);
        }
    }
    
    // Update AUR packages
    println!("Updating AUR packages...");
    let aur_result = Command::new("yay")
        .args(&["-Syu", "--noconfirm", "--noclipboard", "--nodiffmenu", "--noeditmenu"])
        .status();
    
    match aur_result {
        Ok(status) => {
            if status.success() {
                println!("AUR updates completed successfully");
            } else {
                println!("AUR updates failed");
            }
        },
        Err(e) => {
            println!("Failed to execute AUR update: {}", e);
        }
    }
    
    println!("Update process completed");
}