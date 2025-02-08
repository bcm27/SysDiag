use colored::*;
use sysinfo::{ System, SystemExt };

pub fn display_system_info(sys: &System) {
    println!("\n{}", "System Information:".bold().blue());
    println!(
        "Host Name: {}",
        sys.host_name().unwrap_or_else(|| "Unknown".to_string())
    );
    println!(
        "OS: {} {}",
        sys.name().unwrap_or_else(|| "Unknown".to_string()),
        sys.os_version().unwrap_or_else(|| "Unknown".to_string())
    );
    println!(
        "Kernel: {}",
        sys.kernel_version().unwrap_or_else(|| "Unknown".to_string())
    );
}

pub fn handle_system_info() -> Result<(), Box<dyn std::error::Error>> {
    let mut sys = System::new_all();
    sys.refresh_all();
    display_system_info(&sys);
    Ok(())
}
