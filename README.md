# System Diagnostics Rust Program

## Overview

This project is a simple system diagnostics tool written in Rust. It provides users with essential information about their system's performance and health, helping to identify potential issues and optimize system usage. I wrote it because I was tired of trying to figure out my online friends computer specs and it grew from there.

## Features

- **CPU Information**: Displays details about the CPU, including model, cores, and current usage.
- **Memory Usage**: Reports on total, used, and available memory, along with memory usage statistics.
- **Disk Space**: Provides information on disk usage, including total space, used space, and available space for each mounted filesystem.
- **Network Status**: Shows the current network status, including active connections and network interface statistics.
- **Security Diagnostics**: Performs various security checks, including:
  - Checking Windows Defender status
  - Checking Windows Firewall status
  - Checking BitLocker status
  - Checking for Windows updates
  - Checking User Account Control (UAC) status
  - Checking antivirus status
  - Querying recent failed login attempts

## To Compile

To build and run the program, ensure you have Rust installed on your system. You can install Rust using [rustup](https://rustup.rs/).

### Prerequisites
- Ensure you have the latest version of Rust and Cargo installed. You can check your installation by running:
  ```bash
  rustc --version
  cargo --version
  ```

### Build Instructions
1. **Clone the Repository**: 
   Clone the repository to your local machine using:
   ```bash
   git clone https://github.com/yourusername/SysDiag.git
   cd SysDiag
   ```

2. **Build the Project**: 
   Run the following command to build the program and download the necessary dependencies:
   ```bash
   cargo build --release
   ```

3. **Run the Program**: 
   After the build completes, you can run the program with:
   ```bash
   cargo run --release
   ```

### Expected Output
- The program will compile, and you should see output indicating the build process. If successful, you can start using the system diagnostics tool. 