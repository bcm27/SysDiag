use std::process::{Command};

pub fn execute_powershell_command(command: &str) -> Result<String, String> {
    let output = Command::new("powershell")
        .args(["-Command", command])
        .output()
        .map_err(|e| format!("Failed to execute PowerShell command: {}", e))?;
    
    String::from_utf8(output.stdout).map_err(|e| format!("Failed to convert output to String: {}", e))
}
