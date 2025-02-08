use sysinfo::{ System, SystemExt };

pub fn handle_hardware_info() -> Result<(), Box<dyn std::error::Error>> {
    let mut sys = System::new_all();
    sys.refresh_all();

    // Display CPU info
    super::cpu::display_cpu_info(&sys);

    // Display memory info
    super::memory::display_memory_info(&sys);

    // Display disk info
    super::disk::display_disk_info(&sys);

    Ok(())
}
