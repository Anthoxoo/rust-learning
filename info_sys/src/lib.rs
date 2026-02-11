pub mod temperature {
    use std::fs;

    pub fn get_celcius_temperature() -> f32 {
        let path: String = String::from("/sys/class/thermal/thermal_zone0/temp");
        let raw_temperature = fs::read_to_string(path).expect("Error while reading the file");

        let final_temperature: f32 = raw_temperature
            .trim()
            .parse()
            .expect("error while converting to float");
        return final_temperature / 1000.0;
    }
}

pub mod memory {
    use std::fs;

    pub struct RamInfo {
        pub total: u32,
        pub free: u32,
        pub used: u32,
    }
    pub fn get_ram_usage() -> RamInfo {
        let path: String = String::from("/proc/meminfo");
        let reader = fs::read_to_string(path).expect("Error while reading the file");

        let line_total_mem: &str = reader
            .lines()
            .nth(0)
            .expect("This index of line does not exists.");

        let line_free_mem: &str = reader
            .lines()
            .nth(2)
            .expect("This index of line does not exists.");

        let total_mem: u32 = line_total_mem
            .split_whitespace()
            .nth(1)
            .expect("No value found.")
            .parse()
            .expect("Not a number.");

        let free_mem: u32 = line_free_mem
            .split_whitespace()
            .nth(1)
            .expect("No value found.")
            .parse()
            .expect("Not a number.");

        let used_mem: u32 = total_mem - free_mem;

        return RamInfo {
            total: total_mem,
            free: free_mem,
            used: used_mem,
        };
    }
}

pub mod disk {
    use std::process::Command;

    pub struct DiskInfo {
        pub total: u64,
        pub free: u64,
        pub used: u64,
    }
    pub fn get_storage_info() -> DiskInfo {
        let storage_command_output = Command::new("df")
            .arg("-k")
            .arg(".") // shows the main disk
            .output()
            .expect("Error reading df -k command.");

        let stdout_to_string: String = String::from_utf8(storage_command_output.stdout)
            .expect("Error converting to utf-8 df -k command.");

        let disk_line: &str = stdout_to_string
            .lines()
            .nth(1)
            .expect("Error reading line 1 of df -k");

        let total_storage: u64 = disk_line
            .split_whitespace()
            .nth(1)
            .expect("Error reading total storage")
            .parse()
            .expect("Error converting total storage to u64");

        let used_storage: u64 = disk_line
            .split_whitespace()
            .nth(2)
            .expect("Error reading used storage")
            .parse()
            .expect("Error converting used storage to u64");

        let available_storage: u64 = disk_line
            .split_whitespace()
            .nth(3)
            .expect("Error reading available storage")
            .parse()
            .expect("Error converting available storage to u64");

        return DiskInfo {
            total: total_storage,
            free: available_storage,
            used: used_storage,
        };
    }
}
