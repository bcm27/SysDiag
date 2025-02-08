use std::process::Command;

pub fn handle_network_diagnostics() -> Result<(), Box<dyn std::error::Error>> {
    println!("\nNetwork Diagnostics:");

    // Basic network connectivity test
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("ping").args(["-n", "1", "8.8.8.8"]).output()?;
        println!("Network connectivity: {}", String::from_utf8_lossy(&output.stdout));
    }

    #[cfg(not(target_os = "windows"))]
    {
        let output = Command::new("ping").args(["-c", "1", "8.8.8.8"]).output()?;
        println!("Network connectivity: {}", String::from_utf8_lossy(&output.stdout));
    }

    Ok(())
}
