use crate::verbose::say;
use console::style;
use std::{collections::HashMap, fmt};
use sysinfo::{Disks, System};

#[derive(Debug)]
pub struct SystemDisks {
    pub disk_type: Option<String>,
    pub file_system: Option<String>,
    pub free_space: Option<String>,
}

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

    pub fn to_hashmap(&self) -> std::collections::HashMap<String, String> {
        let mut infomap: HashMap<String, String> = HashMap::new();

        infomap.insert("System Name".to_string(), self.system_name.clone());
        infomap.insert(
            "System kernel version".to_string(),
            self.kernel_version.clone(),
        );
        infomap.insert("System OS version".to_string(), self.os_version.clone());
        infomap.insert("Hostname".to_string(), self.hostname.clone());
        infomap.insert("CPU Cores".to_string(), self.cpu_cores.to_string());
        infomap.insert(
            "CPU Virtual Cores".to_string(),
            self.cpu_virtual_cores.to_string(),
        );
        infomap.insert("Total Memory".to_string(), self.total_memory.to_string());
        infomap.insert("Used Memory".to_string(), self.used_memory.to_string());
        infomap.insert("Total Swap".to_string(), self.total_swap.to_string());
        infomap.insert("Used Swap".to_string(), self.used_swap.to_string());

        for disk in &self.disks {
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

    /// Iterates through a HashMap and writes the key-value pairs to the terminal.
    /// You can also pass a second HashMap to add additional key-value pairs.
    /// The keys in the second HashMap will be added to the first one.
    pub fn display(
        &self,
        post_load: Option<HashMap<String, String>>,
    ) -> Result<(), std::io::Error> {
        let lines = self.to_hashmap();

        let mut lines = lines.clone();

        if let Some(notes) = post_load {
            for (key, value) in notes {
                lines.insert(key, value);
            }
        }
        let max_key_length = lines.keys().map(|key| key.len()).max().unwrap_or(0);

        for line in lines {
            let padded_key = format!("{:width$}", line.0, width = max_key_length);

            let keystyle = style(padded_key).cyan();
            let valstyle = style(format!("{}", line.1)).green();
            say(format!("{} :: {}", keystyle, valstyle).as_str())?;
        }
        Ok(())
    }
}

impl fmt::Debug for SystemInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "System Name: {}", self.system_name)?;
        write!(f, "Kernel Version: {}", self.kernel_version)?;
        write!(f, "OS Version: {}", self.os_version)?;
        write!(f, "Hostname: {}", self.hostname)?;
        write!(f, "CPU Cores: {}", self.cpu_cores)?;
        write!(f, "CPU Virtual Cores: {}", self.cpu_virtual_cores)?;
        write!(f, "Total Memory: {}", self.total_memory)?;
        write!(f, "Used Memory: {}", self.used_memory)?;
        write!(f, "Total Swap: {}", self.total_swap)?;
        write!(f, "Used Swap: {}", self.used_swap)?;
        write!(f, "Disks: {:?}", self.disks)
    }
}
