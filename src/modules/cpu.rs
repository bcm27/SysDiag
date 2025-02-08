use colored::*;
use sysinfo::{ System, SystemExt, CpuExt };

pub fn display_cpu_info(sys: &System) {
    println!("\n{}", "CPU Information:".bold().blue());
    println!("CPU Count: {}", sys.cpus().len());

    for (i, cpu) in sys.cpus().iter().enumerate() {
        println!("CPU {}: {}", i + 1, cpu.name());
        println!("CPU {} Usage: {:.2}%", i + 1, cpu.cpu_usage());
    }
}
