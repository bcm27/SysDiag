use colored::*;
use sysinfo::{ System, SystemExt, DiskExt };

pub fn display_disk_info(sys: &System) {
    println!("\n{}", "Disk Information:".bold().blue());
    for disk in sys.disks() {
        println!("Disk Name: {}", disk.name().to_string_lossy());
        println!("File System: {}", String::from_utf8_lossy(disk.file_system()));
        println!("Total Space: {:.2} GB", (disk.total_space() as f64) / 1024.0 / 1024.0 / 1024.0);
        println!(
            "Available Space: {:.2} GB",
            (disk.available_space() as f64) / 1024.0 / 1024.0 / 1024.0
        );
        println!("---");
    }
}
