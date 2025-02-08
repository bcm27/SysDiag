mod modules;

use colored::*;
use std::io::{ Write };
use std::env;
use modules::security::SecurityDiagnostics;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check for command-line arguments for configuration
    let args: Vec<String> = env::args().collect();
    let run_security = args.contains(&String::from("--security"));
    let run_hardware = args.contains(&String::from("--hardware"));
    let run_network = args.contains(&String::from("--network"));

    if run_security {
        handle_security_diagnostics()?;
        return Ok(());
    }

    if run_hardware {
        modules::hardware::handle_hardware_info()?;
        return Ok(());
    }

    if run_network {
        modules::network::handle_network_diagnostics()?;
        return Ok(());
    }

    loop {
        clearscreen::clear()?;
        display_main_menu();
        print!("Enter your choice: ");
        std::io::stdout().flush()?;

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice)?;

        match choice.trim() {
            "0" => {
                modules::system::handle_system_info()?;
                wait_for_user();
            },
            "1" => {
                modules::hardware::handle_hardware_info()?;
                wait_for_user();
            },
            "2" => {
                modules::network::handle_network_diagnostics()?;
                wait_for_user();
            },
            "3" => {
                handle_security_diagnostics()?;
                wait_for_user();
            },
            "4" => {
                clearscreen::clear()?;
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
                std::thread::sleep(std::time::Duration::from_secs(2));
            },
        }
    }
    Ok(())
}

fn display_main_menu() {
    println!("\n{}", "System Diagnostics Menu".bright_green().bold());
    println!("{}", "===================".bright_green());
    println!("{}. {}", "0".yellow(), "System Information");
    println!("{}. {}", "1".yellow(), "Hardware Information");
    println!("{}. {}", "2".yellow(), "Network Diagnostics");
    println!("{}. {}", "3".yellow(), "Security Diagnostics");
    println!("{}. {}", "4".red(), "Exit");
    println!("{}", "===================".bright_green());
    print!("{}", "Enter your choice: ".cyan());
}

fn wait_for_user() {
    println!("\nPress Enter to continue...");
    let _ = std::io::stdin().read_line(&mut String::new());
}

fn handle_security_diagnostics() -> Result<(), Box<dyn std::error::Error>> {
    clearscreen::clear()?;
    let security = SecurityDiagnostics::new();
    println!("\n{}", "Security Diagnostics".bright_blue().bold());
    println!("{}", "===================".bright_blue());

    // Call the method to run all security checks
    security.run_all_security_checks()?;

    println!("{}", "===================".bright_blue());
    Ok(())
}
