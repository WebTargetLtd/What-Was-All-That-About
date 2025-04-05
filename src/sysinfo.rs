use std::collections::HashMap;

use sysinfo::{Disks, System};

#[derive(Debug)]
pub struct SystemDisks {
    pub disk_type: Option<String>,
    pub file_system: Option<String>,
    pub free_space: Option<String>,
}
#[derive(Debug)]
pub struct SystemInfo {
    pub system_name: String,
    pub kernel_version: String,
    pub os_version: String,
    pub hostname: String,
    pub cpu_cores: usize,
    pub cpu_virtual_cores: usize,
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,
    pub disks: Vec<SystemDisks>,
}
impl SystemInfo {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        let disks = Disks::new_with_refreshed_list();

        Self {
            system_name: System::name().unwrap_or_default(),
            kernel_version: System::kernel_version().unwrap_or_default(),
            os_version: System::os_version().unwrap_or_default(),
            hostname: System::host_name().unwrap_or_default(),
            cpu_cores: num_cpus::get_physical(),
            cpu_virtual_cores: num_cpus::get(),
            total_memory: sys.total_memory(),
            used_memory: sys.used_memory(),
            total_swap: sys.total_swap(),
            used_swap: sys.used_swap(),
            disks: disks
                .iter()
                .map(|d| SystemDisks {
                    disk_type: Some(d.kind().to_string()),
                    file_system: d.file_system().to_str().map(|s| s.to_string()),
                    free_space: Some(d.available_space().to_string()),
                })
                .collect(),
        }
    }
pub fn new_to_hashmap(sys_info: Self) -> HashMap<String, String> {
    let mut infomap: HashMap<String, String> = HashMap::new();
    infomap.insert("System Name".to_string(), sys_info.system_name);
    infomap.insert("System kernel version".to_string(), sys_info.kernel_version);
    infomap.insert("System OS version".to_string(), sys_info.os_version);
    infomap.insert("Hostname".to_string(), sys_info.hostname);
    infomap.insert("CPU Cores".to_string(), sys_info.cpu_cores.to_string());
    infomap.insert(
        "CPU Virtual Cores".to_string(),
        sys_info.cpu_virtual_cores.to_string(),
    );
    infomap.insert("Total Memory".to_string(), sys_info.total_memory.to_string());
    infomap.insert("Used Memory".to_string(), sys_info.used_memory.to_string());
    infomap.insert("Total Swap".to_string(), sys_info.total_swap.to_string());
    infomap.insert("Used Swap".to_string(), sys_info.used_swap.to_string());

    for disk in &sys_info.disks {
        if let Some(disk_type) = &disk.disk_type {
            infomap.insert("Disk Type".to_string(), disk_type.clone());
        }
        if let Some(file_system) = &disk.file_system {
            infomap.insert("File System".to_string(), file_system.clone());
        }
        if let Some(free_space) = &disk.free_space {
            infomap.insert("Free Space".to_string(), free_space.clone());
        }
    }

    infomap
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
        println!("Disks: {:?}", self.disks);
        // for disk in &self.disks {
        //     println!(
        //         "Disk Type: {:?}, File System: {:?}, Free Space: {:?}",
        //         disk.disk_type, disk.file_system, disk.free_space
        //     );
        // }
    }
}
