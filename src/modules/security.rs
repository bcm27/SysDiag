use std::process::Command;

pub struct SecurityDiagnostics {
}

impl SecurityDiagnostics {
    pub fn new() -> Self {
        SecurityDiagnostics {
        }
    }

    pub fn query_failed_logins(&self) -> Result<Vec<String>, std::io::Error> {
        #[cfg(target_os = "windows")]
        {
            let output = Command::new("powershell")
                .args([
                    "-Command",
                    "Get-EventLog -LogName Security -InstanceId 4625 -Newest 10 -ErrorAction SilentlyContinue | \
                     Select-Object TimeGenerated, @{Name='Username';Expression={$_.ReplacementStrings[5]}}",
                ])
                .output()?;

            let content = String::from_utf8_lossy(&output.stdout);
            let mut failed_logins = Vec::new();

            for line in content.lines() {
                if !line.trim().is_empty() && !line.contains("TimeGenerated") {
                    failed_logins.push(line.trim().to_string());
                }
            }

            Ok(failed_logins)
        }

        #[cfg(not(target_os = "windows"))]
        {
            Ok(Vec::new())
        }
    }

    // New generic method to execute PowerShell commands
    fn execute_powershell_command(&self, command: &str) -> Result<String, std::io::Error> {
        let output = Command::new("powershell")
            .args(["-Command", command])
            .output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn query_windows_defender_status(&self) -> Result<bool, std::io::Error> {
        #[cfg(target_os = "windows")]
        {
            let status = self.execute_powershell_command("Get-MpComputerStatus | Select-Object AntivirusEnabled")?;
            Ok(status.to_lowercase().contains("true"))
        }
        #[cfg(not(target_os = "windows"))]
        {
            Ok(false)
        }
    }

    pub fn query_windows_firewall_status(&self) -> Result<bool, std::io::Error> {
        #[cfg(target_os = "windows")]
        {
            let status = self.execute_powershell_command("Get-NetFirewallProfile | Select-Object Enabled")?;
            Ok(status.to_lowercase().contains("true"))
        }
        #[cfg(not(target_os = "windows"))]
        {
            Ok(false)
        }
    }

    pub fn query_bitlocker_status(&self) -> Result<bool, std::io::Error> {
        #[cfg(target_os = "windows")]
        {
            let status = self.execute_powershell_command("Get-BitLockerVolume -ErrorAction SilentlyContinue | Select-Object ProtectionStatus")?;
            Ok(status.to_lowercase().contains("on"))
        }
        #[cfg(not(target_os = "windows"))]
        {
            Ok(false)
        }
    }

    pub fn query_windows_updates(&self) -> Result<bool, std::io::Error> {
        #[cfg(target_os = "windows")]
        {
            let status = self.execute_powershell_command("Get-WUList -ErrorAction SilentlyContinue | Measure-Object | Select-Object Count")?;
            let count = status.trim().parse::<i32>().unwrap_or(0);
            Ok(count == 0) // Returns true if updates are available
        }
        #[cfg(not(target_os = "windows"))]
        {
            Ok(false)
        }
    }

    pub fn query_uac_status(&self) -> Result<bool, std::io::Error> {
        #[cfg(target_os = "windows")]
        {
            let status = self.execute_powershell_command("Get-ItemProperty HKLM:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System -Name EnableLUA | Select-Object EnableLUA")?;
            Ok(status.trim().contains("1"))
        }
        #[cfg(not(target_os = "windows"))]
        {
            Ok(false)
        }
    }

    pub fn query_antivirus_status(&self) -> Result<String, std::io::Error> {
        #[cfg(target_os = "windows")]
        {
            let status = self.execute_powershell_command("Get-CimInstance -Namespace root/SecurityCenter2 -ClassName AntivirusProduct | Select-Object displayName, productState")?;
            if status.trim().is_empty() {
                Ok("No antivirus detected".to_string())
            } else {
                // Format the output for better readability
                let mut formatted_status = String::new();
                for (i, line) in status.lines().enumerate() {
                    if i == 1 { // Skip the line with dashes
                        continue;
                    }
                    if !line.trim().is_empty() {
                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() >= 2 {
                            formatted_status.push_str(
                                &format!("{}: Version {}\n", parts[0], parts[1])
                            );
                        }
                    }
                }
                Ok(formatted_status.trim().to_string())
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            Ok("Antivirus check not implemented for this OS".to_string())
        }
    }

    pub fn run_all_security_checks(&self) -> Result<(), std::io::Error> {
        println!("\n=== Security Check Results ===");

        self.check_windows_defender()?;
        self.check_firewall()?;
        self.check_bitlocker()?;
        self.check_windows_updates()?;
        self.check_uac()?;
        self.check_antivirus()?;
        self.check_failed_logins()?;

        Ok(())
    }

    fn check_windows_defender(&self) -> Result<(), std::io::Error> {
        match self.query_windows_defender_status() {
            Ok(true) => println!("✅ Windows Defender: Active"),
            Ok(false) => println!("❌ Windows Defender: Not Active"),
            Err(e) => println!("❗ Windows Defender Check Error: {}", e),
        }
        Ok(())
    }

    fn check_firewall(&self) -> Result<(), std::io::Error> {
        match self.query_windows_firewall_status() {
            Ok(true) => println!("✅ Windows Firewall: Enabled"),
            Ok(false) => println!("❌ Windows Firewall: Disabled"),
            Err(e) => println!("❗ Firewall Check Error: {}", e),
        }
        Ok(())
    }

    fn check_bitlocker(&self) -> Result<(), std::io::Error> {
        match self.query_bitlocker_status() {
            Ok(true) => println!("✅ BitLocker: Enabled"),
            Ok(false) => println!("❌ BitLocker: Not Enabled"),
            Err(e) => println!("❗ BitLocker Check Error: {}", e),
        }
        Ok(())
    }

    fn check_windows_updates(&self) -> Result<(), std::io::Error> {
        match self.query_windows_updates() {
            Ok(true) => println!("✅ Windows Updates: System Up to Date"),
            Ok(false) => println!("❗ Windows Updates: Updates Available"),
            Err(e) => println!("❗ Update Check Error: {}", e),
        }
        Ok(())
    }

    fn check_uac(&self) -> Result<(), std::io::Error> {
        match self.query_uac_status() {
            Ok(true) => println!("✅ UAC: Enabled"),
            Ok(false) => println!("❌ UAC: Disabled"),
            Err(e) => println!("❗ UAC Check Error: {}", e),
        }
        Ok(())
    }

    fn check_antivirus(&self) -> Result<(), std::io::Error> {
        match self.query_antivirus_status() {
            Ok(status) => {
                println!("Antivirus Status:\n{}", status);
            }
            Err(e) => println!("❗ Error checking antivirus status: {}", e),
        }
        Ok(())
    }

    fn check_failed_logins(&self) -> Result<(), std::io::Error> {
        match self.query_failed_logins() {
            Ok(logins) => {
                println!("\n🔐 Recent Failed Login Attempts:");
                if logins.is_empty() {
                    println!("  No recent failed login attempts");
                } else {
                    for login in logins {
                        println!("  - {}", login);
                    }
                }
            }
            Err(e) => println!("❗ Error checking failed logins: {}", e),
        }
        Ok(())
    }
}
