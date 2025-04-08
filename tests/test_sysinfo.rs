#[cfg(test)]
mod tests {

    use wolves_cli_helper::sysinfo::SystemInfo;
    #[test]
    fn test_system_info_new() {
        let sys_info = SystemInfo::new();

        // Check that the fields are not empty or invalid
        assert!(!sys_info.system_name.is_empty());
        assert!(!sys_info.kernel_version.is_empty());
        assert!(!sys_info.os_version.is_empty());
        assert!(!sys_info.hostname.is_empty());
        assert!(sys_info.cpu_cores > 0);
        assert!(sys_info.cpu_virtual_cores > 0);
        assert!(sys_info.total_memory > 0);
        assert!(sys_info.used_memory <= sys_info.total_memory);
        assert!(sys_info.total_swap >= sys_info.used_swap);
    }

    #[test]
    fn test_to_hashmap() {
        let sys_info = SystemInfo::new();
        let hashmap = sys_info.to_hashmap();

        // Check that the hashmap contains expected keys
        assert!(hashmap.contains_key("System Name"));
        assert!(hashmap.contains_key("System kernel version"));
        assert!(hashmap.contains_key("System OS version"));
        assert!(hashmap.contains_key("Hostname"));
        assert!(hashmap.contains_key("CPU Cores"));
        assert!(hashmap.contains_key("CPU Virtual Cores"));
        assert!(hashmap.contains_key("Total Memory"));
        assert!(hashmap.contains_key("Used Memory"));
        assert!(hashmap.contains_key("Total Swap"));
        assert!(hashmap.contains_key("Used Swap"));
    }
}
/* 
    #[test]
    fn test_display() {
        let sys_info = SystemInfo::new();
        let result = sys_info.display(None);

        // Ensure the display method does not return an error
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_with_post_load() {
        let sys_info = SystemInfo::new();
        let mut post_load = HashMap::new();
        post_load.insert("Custom Key".to_string(), "Custom Value".to_string());

        let result = sys_info.display(Some(post_load));

        // Ensure the display method does not return an error
        assert!(result.is_ok());
    }
}
    
*/