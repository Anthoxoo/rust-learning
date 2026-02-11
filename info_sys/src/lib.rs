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
        const KO_TO_MO: u32 = 1024;

        return RamInfo {
            total: total_mem / KO_TO_MO,
            free: free_mem / KO_TO_MO,
            used: used_mem / KO_TO_MO,
        };
    }
}

pub mod disk {
    use std::process::Command;

    pub struct DiskInfo {
        pub total: u64,
        pub free: u64,
        pub free_percentage: u64,
        pub used: u64,
        pub used_percentage: u64,
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

        const KB_TO_GO: u64 = 1024 * 1024;
        let free_disk_pourcentage: u64 = (available_storage * 100) / total_storage;
        let used_disk_pourcentage: u64 = (used_storage * 100) / total_storage;

        return DiskInfo {
            total: total_storage / KB_TO_GO,
            free: available_storage / KB_TO_GO,
            free_percentage: free_disk_pourcentage,
            used: used_storage / KB_TO_GO,
            used_percentage: used_disk_pourcentage,
        };
    }
}

pub mod os {
    use std::fs;

    pub fn name() -> String {
        let path: String = String::from("/etc/os-release");
        let raw_os_output: String =
            fs::read_to_string(path).expect("Error reading /etc/os-release.");
        let classic_name: &str = raw_os_output
            .lines()
            .nth(0)
            .expect("Error reading first line");
        let mut pretty_name: &str = ""; // no need mut because &str --> on the stack

        for line in raw_os_output.lines() {
            if line.starts_with("PRETTY_NAME") {
                pretty_name = line;
                break;
            }
        }

        if pretty_name == r#"PRETTY_NAME="Linux""# {
            //case where the pretty name has not been changed ( rare case )
            pretty_name = classic_name
        }

        pretty_name = pretty_name
            .split('=')
            .nth(1)
            .expect("Error splitting the classic name on the '='")
            .trim();
        return pretty_name.to_string();
    }

    pub fn uptime() -> f64 {
        let path: String = String::from("/proc/uptime");
        let raw_uptime_output: String =
            fs::read_to_string(path).expect("Error reading /proc/uptime.");

        let uptime_output: f64 = raw_uptime_output
            .split(' ')
            .nth(0)
            .expect("Error splitting on the space")
            .parse()
            .expect("Error converting to float");
        const SEC_TO_HOUR: f64 = 60.0 * 60.0;
        return uptime_output / SEC_TO_HOUR;
    }
}
