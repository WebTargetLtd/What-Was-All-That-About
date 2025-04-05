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

#[derive(Debug)]
pub struct SystemInfo {
    pub system_name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub hostname: String,
    pub cpu_cores: String,
    pub cpu_virtual_cores: String,
    pub total_memory: String,
    pub used_memory: String,
    pub total_swap: String,
    pub used_swap: String,
    pub disk_type: Option<String>,
    pub file_system: Option<String>,
    pub free_space: Option<String>,
}
impl SystemInfo {
    pub fn new() -> Self {
        Self {
            system_name: String::new(),
            kernel_version: String::new(),
            os_version: String::new(),
            hostname: String::new(),
            cpu_cores: String::new(),
            cpu_virtual_cores: String::new(),
            total_memory: String::new(),
            used_memory: String::new(),
            total_swap: String::new(),
            used_swap: String::new(),
            disk_type: None,
            file_system: None,
            free_space: None,
        }
    }
    pub fn from_map(map: HashMap<&str, String>) -> Self {
        let mut system_info = SystemInfo::new();
        for (key, value) in map.iter() {
            match *key {
                "System Name" => system_info.system_name = value.clone(),
                "System kernel version" => system_info.kernel_version = value.clone(),
                "System OS version" => system_info.os_version = value.clone(),
                "Hostname" => system_info.hostname = value.clone(),
                "CPU Cores" => system_info.cpu_cores = value.clone(),
                "CPU Virtual Cores" => system_info.cpu_virtual_cores = value.clone(),
                "Total Memory" => system_info.total_memory = value.clone(),
                "Used Memory" => system_info.used_memory = value.clone(),
                "Total Swap" => system_info.total_swap = value.clone(),
                "Used Swap" => system_info.used_swap = value.clone(),
                _ => {}
            }
        }
        system_info
    }
    pub fn from_sysinfo(sys: &System) -> Self {
        let mut system_info = SystemInfo::new();
        system_info.system_name = System::name().unwrap_or_default();
        system_info.kernel_version = System::kernel_version().unwrap_or_default();
        system_info.os_version = System::os_version().unwrap_or_default();
        system_info.hostname = System::host_name().unwrap_or_default();
        system_info.cpu_cores = num_cpus::get_physical().to_string();
        system_info.cpu_virtual_cores = num_cpus::get().to_string();
        system_info.total_memory = sys.total_memory().to_string();
        system_info.used_memory = sys.used_memory().to_string();
        system_info.total_swap = sys.total_swap().to_string();
        system_info.used_swap = sys.used_swap().to_string();

        system_info
    }
    pub fn from_disks(disks: &Disks) -> Self {
        let mut system_info = SystemInfo::new();
        for disk in disks {
            system_info.disk_type = Some(disk.kind().to_string());
            system_info.file_system = Some(disk.file_system().to_str().unwrap_or_default().to_string());
            system_info.free_space = Some(disk.available_space().to_string());
        }
        system_info
    }
    pub fn display(&self) {
        println!("System Name: {}", self.system_name);
        println!("Kernel Version: {}", self.kernel_version);
        println!("OS Version: {}", self.os_version);
        println!("Hostname: {}", self.hostname);
        println!("CPU Cores: {}", self.cpu_cores);
        println!("CPU Virtual Cores: {}", self.cpu_virtual_cores);
        println!("Total Memory: {}", self.total_memory);
        println!("Used Memory: {}", self.used_memory);
        println!("Total Swap: {}", self.total_swap);
        println!("Used Swap: {}", self.used_swap);

        if let Some(disk_type) = &self.disk_type {
            println!("Disk Type: {}", disk_type);
        }
        if let Some(file_system) = &self.file_system {
            println!("File System: {}", file_system);
        }
        if let Some(free_space) = &self.free_space {
            println!("Free Space: {}", free_space);
        }
    }
    pub fn to_map(&self) -> HashMap<&str, String> {
        let mut map = HashMap::new();
        map.insert("System Name", self.system_name.clone());
        map.insert("Kernel Version", self.kernel_version.clone());
        map.insert("OS Version", self.os_version.clone());
        map.insert("Hostname", self.hostname.clone());
        map.insert("CPU Cores", self.cpu_cores.clone());
        map.insert("CPU Virtual Cores", self.cpu_virtual_cores.clone());
        map.insert("Total Memory", self.total_memory.clone());
        map.insert("Used Memory", self.used_memory.clone());
        map.insert("Total Swap", self.total_swap.clone());
        map.insert("Used Swap", self.used_swap.clone());

        if let Some(disk_type) = &self.disk_type {
            map.insert("Disk Type", disk_type.clone());
        }
        if let Some(file_system) = &self.file_system {
            map.insert("File System", file_system.clone());
        }
        if let Some(free_space) = &self.free_space {
            map.insert("Free Space", free_space.clone());
        }
        map
    }
}   