/*!
 * # Verbose Utilities Module
 *
 * This module provides utility functions for displaying verbose output
 * in the terminal. It is designed to enhance the user experience by
 * providing formatted and styled messages, system information, and
 * configuration details.
 *
 * ## Features
 * - **say(message: String)**: Prints a styled message to the terminal with a timestamp.
 * - **announce()**: Displays detailed system information, including CPU, memory, swap, and disk details, as well as the configuration file being used.
 * - **writeheaderlines(lines: HashMap<&str, String>)**: Formats and aligns key-value pairs for terminal output.
 * - **paddingline()**: Prints a styled separator line for better readability in the terminal.
 *
 * ## Dependencies
 * - `chrono`: For generating timestamps.
 * - `console`: For styling terminal output.
 * - `sysinfo`: For retrieving system information such as CPU, memory, and disk details.
 *
 * ## Usage
 * This module is intended to be used in applications where detailed system
 * information and verbose output are required, such as debugging or logging
 * utilities.
 *
 * ## Example
 * 
 * use utilities::verbose::{say, announce};
 *
 * fn main() {
 *     say("Starting the application...".to_string()).unwrap();
 *     announce();
 * }
 * 
 */

use std::collections::HashMap;
use chrono::Utc;
use console::{Term, style};
use sysinfo::{Disks, System};

// use super::config::config::report_configfile;

///
/// Prints a styled message to the terminal with a timestamp.
/// 
/// ```rust
///     say(format!("X is : {}", x).as_str()).unwrap();
/// 
/// ```
/// 
pub fn say(message: &str) -> Result<(), std::io::Error> {
    let term = Term::stdout();
    let now = Utc::now();

    let lead = style(format!("[ {} ] :: ", now.to_rfc2822(),)).color256(208);

    let msg = format!("{}{}", lead, message);
    term.write_line(&msg)?;
    Ok(())
}

pub fn announce(preload: Option<HashMap<&str, String>>) {

    // First we update all information of our `System` struct.
    let mut sys = System::new_all();
    sys.refresh_all();

    let num_cores = num_cpus::get();
    let num_physical_cores = num_cpus::get_physical();
    let mut infomap: HashMap<&str, String> = HashMap::new();
    
    if preload.is_some() {
        let preload = preload.unwrap();
        infomap.extend(preload.clone());
        for (key, value) in preload.iter() {
            println!("{}: {}", key, value);
        }
    }
  
    infomap.insert("System Name", System::name().unwrap());
    infomap.insert("System kernel version", System::kernel_version().unwrap());
    infomap.insert("System OS version", System::os_version().unwrap());
    infomap.insert("Hostname", System::host_name().unwrap());

    infomap.insert("CPU Cores", num_physical_cores.to_string());
    infomap.insert("CPU Virtual Cores", num_cores.to_string());
    infomap.insert("Total Memory", sys.total_memory().to_string());
    infomap.insert("Used Memory", sys.used_memory().to_string());
    infomap.insert("Total Swap", sys.total_swap().to_string());
    infomap.insert("Used Swap", sys.used_swap().to_string());
    infomap.insert("Total Memory", sys.total_memory().to_string());

    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        infomap.insert("Disk Type", disk.kind().to_string());
        infomap.insert(
            "File System",
            disk.file_system().to_str().unwrap().to_string(),
        );
        infomap.insert("Free Space", disk.available_space().to_string());
    }

    paddingline();
    writeheaderlines(infomap).unwrap();
    paddingline();
}

/// Iterates through a HashMap and writes the key-value pairs to the terminal.
/// ```rust
/// 
///     // Example usage:
///     let mut infomap: HashMap<&str, String> = HashMap::new();
///     infomap.insert("Using .ini file", report_configfile());
///     infomap.insert("System Name", System::name().unwrap());
///     infomap.insert("System kernel version", System::kernel_version().unwrap());
///     infomap.insert("System OS version", System::os_version().unwrap());
///     infomap.insert("Hostname", System::host_name().unwrap());
/// 
///     // Call the function with our HashMap
///     writeheaderlines(infomap).unwrap();
/// ```
/// 
fn writeheaderlines(lines: HashMap<&str, String>) -> Result<(), std::io::Error> {
    let term = Term::stdout();
    let max_key_length = lines.keys().map(|key| key.len()).max().unwrap_or(0);

    for line in lines {
        let padded_key = format!("{:width$}", line.0, width = max_key_length);

        let keystyle = style(padded_key).color256(208);

        let valstyle = style(format!("{}", line.1)).green();
        term.write_line(format!("{} :: {}", keystyle, valstyle).as_str())?
    }
    Ok(())
}
/// Writes a padding line to the terminal, preceded and followed by a blank line.
pub fn paddingline() {
    let blankline = "";
    let line = style("-----------------------------------------------------------------").blue();

    let term = Term::stdout();

    term.write_line(blankline).unwrap();
    term.write_line(format!("{}", line).as_str()).unwrap();
    term.write_line(blankline).unwrap();
}
