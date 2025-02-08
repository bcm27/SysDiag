use colored::*;
use sysinfo::{ System, SystemExt };

pub fn display_memory_info(sys: &System) {
    println!("\n{}", "Memory Information:".bold().blue());
    println!("Total Memory: {:.2} GB", (sys.total_memory() as f64) / 1024.0 / 1024.0);
    println!("Used Memory: {:.2} GB", (sys.used_memory() as f64) / 1024.0 / 1024.0);
    println!("Available Memory: {:.2} GB", (sys.available_memory() as f64) / 1024.0 / 1024.0);
    println!("Total Swap: {:.2} GB", (sys.total_swap() as f64) / 1024.0 / 1024.0);
    println!("Used Swap: {:.2} GB", (sys.used_swap() as f64) / 1024.0 / 1024.0);
}
